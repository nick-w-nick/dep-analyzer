use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportAnalysis {
    pub file_path: PathBuf,
    pub package_imports: Vec<String>,
    pub cross_feature_imports: Vec<String>,
    pub local_imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub total_files: usize,
    pub files_with_packages: usize,
    pub files_with_cross_feature: usize,
    pub unique_packages: Vec<(String, usize)>, // (import_name, occurrence_count)
    pub unique_cross_feature: Vec<(String, usize)>, // (import_name, occurrence_count)
    pub package_count: usize,
    pub cross_feature_count: usize,
}

#[derive(Debug)]
pub enum ImportType {
    Package,
    CrossFeature,
    Local,
}
