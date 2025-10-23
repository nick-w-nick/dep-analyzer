use crate::models::{ImportAnalysis, ImportType};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct DependencyAnalyzer {
    target_dir: PathBuf,
    import_regex: Regex,
    alias_symbol: String,
}

impl DependencyAnalyzer {
    pub fn new(target_dir: PathBuf, alias_symbol: String) -> Result<Self, Box<dyn std::error::Error>> {
        let import_regex = Regex::new(
            r#"(?:import|export).*?from\s+['"`]([^'"`]+)['"`]|import\s+['"`]([^'"`]+)['"`]"#
        )?;

        Ok(Self {
            target_dir: target_dir.canonicalize()?,
            import_regex,
            alias_symbol,
        })
    }

    pub fn analyze(&self) -> Result<Vec<ImportAnalysis>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for entry in WalkDir::new(&self.target_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| self.is_valid_file(e.path()))
        {
            let analysis = self.analyze_file(entry.path())?;
            results.push(analysis);
        }

        Ok(results)
    }

    fn analyze_file(&self, file_path: &Path) -> Result<ImportAnalysis, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut package_imports = Vec::new();
        let mut cross_feature_imports = Vec::new();
        let mut local_imports = Vec::new();

        for cap in self.import_regex.captures_iter(&content) {
            let import_path = cap.get(1).or_else(|| cap.get(2))
                .map(|m| m.as_str())
                .unwrap_or("");

            if import_path.is_empty() {
                continue;
            }

            match self.classify_import(import_path) {
                ImportType::Package => package_imports.push(import_path.to_string()),
                ImportType::CrossFeature => cross_feature_imports.push(import_path.to_string()),
                ImportType::Local => local_imports.push(import_path.to_string()),
            }
        }

        package_imports.sort();
        package_imports.dedup();
        cross_feature_imports.sort();
        cross_feature_imports.dedup();
        local_imports.sort();
        local_imports.dedup();

        Ok(ImportAnalysis {
            file_path: file_path.to_path_buf(),
            package_imports,
            cross_feature_imports,
            local_imports,
        })
    }

    fn classify_import(&self, import_path: &str) -> ImportType {
        // Package import (no relative path indicators and no alias)
        if !import_path.starts_with('.') && !import_path.starts_with('/') && !import_path.starts_with(&self.alias_symbol) {
            return ImportType::Package;
        }

        // Aliased imports are cross-feature dependencies
        if import_path.starts_with(&self.alias_symbol) {
            return ImportType::CrossFeature;
        }

        // Relative imports are local to the feature
        ImportType::Local
    }


    fn is_valid_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext, "ts" | "tsx" | "js" | "jsx"))
            .unwrap_or(false)
    }
}
