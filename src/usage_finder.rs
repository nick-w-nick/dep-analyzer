use crate::models::ImportAnalysis;
use globset::Glob;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct UsageResult {
    pub file_path: PathBuf,
    pub matching_imports: Vec<String>,
}

pub fn find_external_usage(
    results: &[ImportAnalysis],
    pattern: &str,
) -> Result<Vec<UsageResult>, Box<dyn std::error::Error>> {
    // Build glob matcher from pattern
    let glob = Glob::new(pattern)?;
    let matcher = glob.compile_matcher();

    // Also build a matcher to check if a file is within the pattern path
    // Extract the base path from the pattern (e.g., "~/Meetings/components/details/*" -> "~/Meetings/components/details/")
    let pattern_base = extract_base_path(pattern);
    let base_matcher = if let Some(base) = &pattern_base {
        let base_glob = Glob::new(&format!("{}**/*", base))?;
        Some(base_glob.compile_matcher())
    } else {
        None
    };

    let mut usage_map: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for analysis in results {
        // Check if this file is within the pattern path itself (should be excluded)
        // Skip files that are within the pattern base path
        if let Some(ref base_match) = base_matcher {
            if base_match.is_match(&analysis.file_path) {
                continue;
            }
        }

        // Check all imports (package, cross-feature, and local)
        let all_imports: Vec<&String> = analysis
            .package_imports
            .iter()
            .chain(analysis.cross_feature_imports.iter())
            .chain(analysis.local_imports.iter())
            .collect();

        let mut matching_imports = Vec::new();

        for import in all_imports {
            if matcher.is_match(import) {
                matching_imports.push(import.clone());
            }
        }

        if !matching_imports.is_empty() {
            usage_map.insert(analysis.file_path.clone(), matching_imports);
        }
    }

    // Convert to sorted results
    let mut results: Vec<UsageResult> = usage_map
        .into_iter()
        .map(|(file_path, matching_imports)| UsageResult {
            file_path,
            matching_imports,
        })
        .collect();

    // Sort by file path for consistent output
    results.sort_by(|a, b| a.file_path.cmp(&b.file_path));

    Ok(results)
}

/// Extract the base path from a glob pattern
/// e.g., "~/Meetings/components/details/*" -> Some("~/Meetings/components/details/")
/// e.g., "~/legacy/**/*" -> Some("~/legacy/")
fn extract_base_path(pattern: &str) -> Option<String> {
    // Find the position of the first glob character
    let glob_chars = ['*', '?', '[', '{'];
    let first_glob_pos = pattern.chars().position(|c| glob_chars.contains(&c))?;

    // Get the substring before the first glob character
    let base = &pattern[..first_glob_pos];

    // Remove trailing slash if present
    let base = base.trim_end_matches('/');

    // Add trailing slash back for matching
    Some(format!("{}/", base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_base_path() {
        assert_eq!(
            extract_base_path("~/Meetings/components/details/*"),
            Some("~/Meetings/components/details/".to_string())
        );
        assert_eq!(
            extract_base_path("~/legacy/**/*"),
            Some("~/legacy/".to_string())
        );
        assert_eq!(
            extract_base_path("@/utils/*"),
            Some("@/utils/".to_string())
        );
    }
}
