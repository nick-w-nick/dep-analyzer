use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Threshold for automatic paging (number of lines)
const PAGING_THRESHOLD: usize = 30;

pub struct Pager {
    buffer: Vec<u8>,
}

impl Pager {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    /// Finish writing and display output, using a pager if appropriate
    pub fn finish(self) -> io::Result<()> {
        // If output is being redirected, just write directly
        if !atty::is(atty::Stream::Stdout) {
            io::stdout().write_all(&self.buffer)?;
            return Ok(());
        }

        // Count lines in buffer
        let line_count = self.buffer.iter().filter(|&&b| b == b'\n').count();

        // If output is small enough, just print directly
        if line_count < PAGING_THRESHOLD {
            io::stdout().write_all(&self.buffer)?;
            return Ok(());
        }

        // Try to use a pager
        self.use_pager()
    }

    fn use_pager(self) -> io::Result<()> {
        // Try common pagers in order of preference
        let pagers = ["less", "more", "cat"];

        for pager_cmd in &pagers {
            let mut child = match Command::new(pager_cmd)
                .stdin(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(_) => continue,
            };

            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(&self.buffer).is_ok() {
                    drop(stdin); // Close stdin to signal EOF
                    let _ = child.wait();
                    return Ok(());
                }
            }
        }

        // If all pagers failed, just write to stdout
        io::stdout().write_all(&self.buffer)?;
        Ok(())
    }
}

impl Write for Pager {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
