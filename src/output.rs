use crate::models::{ImportAnalysis, Summary};
use crate::usage_finder::UsageResult;
use std::io::{self, Write};

pub fn print_summary<W: Write>(summary: &Summary, writer: &mut W) -> io::Result<()> {
    writeln!(writer, "=== DEPENDENCY ANALYSIS SUMMARY ===")?;
    writeln!(writer, "Total files analyzed: {}", summary.total_files)?;
    writeln!(writer, "Files with package imports: {}", summary.files_with_packages)?;
    writeln!(writer, "Files with cross-feature imports: {}", summary.files_with_cross_feature)?;
    writeln!(writer)?;

    writeln!(writer, "Package Dependencies ({}):", summary.package_count)?;
    for (pkg, count) in &summary.unique_packages {
        writeln!(writer, "  • {} ({} occurrence{})", pkg, count, if *count == 1 { "" } else { "s" })?;
    }
    writeln!(writer)?;

    writeln!(writer, "Cross-Feature Dependencies ({}):", summary.cross_feature_count)?;
    for (ext, count) in &summary.unique_cross_feature {
        writeln!(writer, "  • {} ({} occurrence{})", ext, count, if *count == 1 { "" } else { "s" })?;
    }
    Ok(())
}

pub fn print_table<W: Write>(results: &[ImportAnalysis], show_local: bool, external_only: bool, writer: &mut W) -> io::Result<()> {
    // Filter results based on external_only flag
    let filtered: Vec<_> = results.iter()
        .filter(|analysis| {
            if external_only {
                !analysis.package_imports.is_empty() || !analysis.cross_feature_imports.is_empty()
            } else {
                true
            }
        })
        .collect();

    if filtered.is_empty() {
        writeln!(writer, "No files match the criteria.")?;
        return Ok(());
    }

    // Calculate maximum file path length for column width
    let max_path_len = filtered.iter()
        .map(|a| {
            let relative_path = a.file_path.strip_prefix(std::env::current_dir().unwrap_or_default())
                .unwrap_or(&a.file_path);
            relative_path.display().to_string().len()
        })
        .max()
        .unwrap_or(20)
        .min(60); // Cap at 60 characters

    // Print header
    write!(writer, "┌{:─<width$}┬{:─<8}┬{:─<13}", "", "", "", width = max_path_len + 2)?;
    if show_local {
        write!(writer, "┬{:─<7}", "")?;
    }
    writeln!(writer, "┐")?;

    write!(writer, "│ {:width$} │ {:^6} │ {:^11}", "File", "Pkgs", "Cross-Feat", width = max_path_len)?;
    if show_local {
        write!(writer, " │ {:^5}", "Local")?;
    }
    writeln!(writer, " │")?;

    write!(writer, "├{:─<width$}┼{:─<8}┼{:─<13}", "", "", "", width = max_path_len + 2)?;
    if show_local {
        write!(writer, "┼{:─<7}", "")?;
    }
    writeln!(writer, "┤")?;

    // Print rows
    for analysis in &filtered {
        let relative_path = analysis.file_path.strip_prefix(std::env::current_dir().unwrap_or_default())
            .unwrap_or(&analysis.file_path);

        let path_str = relative_path.display().to_string();
        let truncated_path = if path_str.len() > max_path_len {
            format!("...{}", &path_str[path_str.len() - max_path_len + 3..])
        } else {
            path_str.clone()
        };

        let pkg_count = analysis.package_imports.len();
        let cross_count = analysis.cross_feature_imports.len();
        let local_count = analysis.local_imports.len();

        write!(writer, "│ {:width$} │ {:^6} │ {:^11}", truncated_path, pkg_count, cross_count, width = max_path_len)?;
        if show_local {
            write!(writer, " │ {:^5}", local_count)?;
        }
        writeln!(writer, " │")?;
    }

    // Print footer
    write!(writer, "└{:─<width$}┴{:─<8}┴{:─<13}", "", "", "", width = max_path_len + 2)?;
    if show_local {
        write!(writer, "┴{:─<7}", "")?;
    }
    writeln!(writer, "┘")?;
    Ok(())
}

pub fn print_usage<W: Write>(results: &[UsageResult], pattern: &str, writer: &mut W) -> io::Result<()> {
    if results.is_empty() {
        writeln!(writer, "No external dependencies found for pattern: {}", pattern)?;
        writeln!(writer, "\nThis means no files outside the pattern are importing from it.")?;
        writeln!(writer, "Safe to delete! ✓")?;
        return Ok(());
    }

    writeln!(writer, "=== EXTERNAL DEPENDENCIES ON: {} ===", pattern)?;
    writeln!(writer, "\nFound {} file{} importing from this path:\n",
        results.len(),
        if results.len() == 1 { "" } else { "s" }
    )?;

    for usage in results {
        let relative_path = usage.file_path
            .strip_prefix(std::env::current_dir().unwrap_or_default())
            .unwrap_or(&usage.file_path);

        writeln!(writer, "📄 {}", relative_path.display())?;

        for import in &usage.matching_imports {
            writeln!(writer, "   • {}", import)?;
        }
        writeln!(writer)?;
    }

    writeln!(writer, "Total: {} external dependencies", results.len())?;
    writeln!(writer, "\n⚠️  These files need to be updated before removing the legacy code.")?;

    Ok(())
}
