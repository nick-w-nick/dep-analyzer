use crate::models::{ImportAnalysis, Summary};
use std::collections::HashMap;

pub fn generate_summary(results: &[ImportAnalysis]) -> Summary {
    let mut package_counts: HashMap<String, usize> = HashMap::new();
    let mut cross_feature_counts: HashMap<String, usize> = HashMap::new();
    let mut files_with_packages = 0;
    let mut files_with_cross_feature = 0;

    for analysis in results {
        if !analysis.package_imports.is_empty() {
            files_with_packages += 1;
            for pkg in &analysis.package_imports {
                *package_counts.entry(pkg.clone()).or_insert(0) += 1;
            }
        }
        if !analysis.cross_feature_imports.is_empty() {
            files_with_cross_feature += 1;
            for cross in &analysis.cross_feature_imports {
                *cross_feature_counts.entry(cross.clone()).or_insert(0) += 1;
            }
        }
    }

    // Convert to sorted vectors with counts
    let mut unique_packages: Vec<(String, usize)> = package_counts.into_iter().collect();
    let mut unique_cross_feature: Vec<(String, usize)> = cross_feature_counts.into_iter().collect();

    // Sort by name
    unique_packages.sort_by(|a, b| a.0.cmp(&b.0));
    unique_cross_feature.sort_by(|a, b| a.0.cmp(&b.0));

    Summary {
        total_files: results.len(),
        files_with_packages,
        files_with_cross_feature,
        package_count: unique_packages.len(),
        cross_feature_count: unique_cross_feature.len(),
        unique_packages,
        unique_cross_feature,
    }
}
