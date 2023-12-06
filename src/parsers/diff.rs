use regex::Regex;

use crate::entities::diff::{DiffChange, FileDiff};

pub struct DiffParser {}

impl DiffParser {
    pub fn new() -> Self {
        DiffParser {}
    }

    pub fn parse_file_diff(&self, diff_file: &str) -> FileDiff {
        let re = Regex::new(r"@@ -\d+,\d+ \+\d+,\d+ @@").unwrap();
        let mut start = 0;

        let mut changes: Vec<DiffChange> = Vec::new();
        for mat in re.find_iter(diff_file) {
            let matched_text = &diff_file[start..mat.start()];
            match self.reconstruct_versions(matched_text) {
                Some(change) => changes.push(change),
                None => continue,
            }
            start = mat.end();
        }

        if start < diff_file.len() {
            let remaining_text = &diff_file[start..];
            match self.reconstruct_versions(remaining_text) {
                Some(change) => changes.push(change),
                None => {}
            }
        }

        FileDiff { changes: changes }
    }

    fn reconstruct_versions(&self, diff_text: &str) -> Option<DiffChange> {
        let mut removed_version = String::new();
        let mut added_version = String::new();

        for line in diff_text.lines() {
            if line.starts_with("-") {
                removed_version.push_str(&line[1..]);
                removed_version.push('\n');
            } else if line.starts_with("+") {
                added_version.push_str(&line[1..]);
                added_version.push('\n');
            }
        }

        if added_version.is_empty() && removed_version.is_empty() {
            return None;
        }

        Some(DiffChange {
            added_code: added_version,
            removed_code: removed_version,
        })
    }
}
