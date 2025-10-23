use clap::Parser;
use dep_analyzer::{generate_summary, print_summary, print_table, DependencyAnalyzer, Pager};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dep-analyzer")]
#[command(about = "Analyze TypeScript/JavaScript import dependencies")]
struct Args {
    /// Target directory to analyze (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    /// Output format: table, json, summary
    #[arg(short, long, default_value = "summary")]
    format: String,

    /// Show local imports in detailed output
    #[arg(long)]
    show_local: bool,

    /// Only show files with external dependencies
    #[arg(long)]
    external_only: bool,

    /// Symbol used for cross-feature imports (aliased imports)
    #[arg(long, default_value = "~")]
    alias_symbol: String,

    /// Write output to a file instead of stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let analyzer = DependencyAnalyzer::new(args.dir, args.alias_symbol)?;
    let results = analyzer.analyze()?;

    if results.is_empty() {
        println!("No TypeScript/JavaScript files found in the target directory.");
        return Ok(());
    }

    // Determine output destination
    if let Some(output_path) = args.output {
        // Write to file
        let mut file = fs::File::create(&output_path)?;

        match args.format.as_str() {
            "json" => {
                let summary = generate_summary(&results);
                writeln!(file, "{}", serde_json::to_string_pretty(&summary)?)?;
            }
            "table" => {
                print_table(&results, args.show_local, args.external_only, &mut file)?;
            }
            "summary" | _ => {
                let summary = generate_summary(&results);
                print_summary(&summary, &mut file)?;
            }
        }

        eprintln!("Output written to: {}", output_path.display());
    } else {
        // Use pager for stdout (automatically pages if output is large and TTY)
        let mut pager = Pager::new();

        match args.format.as_str() {
            "json" => {
                let summary = generate_summary(&results);
                writeln!(pager, "{}", serde_json::to_string_pretty(&summary)?)?;
            }
            "table" => {
                print_table(&results, args.show_local, args.external_only, &mut pager)?;
            }
            "summary" | _ => {
                let summary = generate_summary(&results);
                print_summary(&summary, &mut pager)?;
            }
        }

        pager.finish()?;
    }

    Ok(())
}
