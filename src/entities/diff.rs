#[derive(Debug)]
pub struct DiffLineChange {
    pub removed: String,
    pub added: String,
}

#[derive(Debug)]
pub struct DiffChange {
    pub line_changes: Vec<DiffLineChange>,
}

#[derive(Debug)]
pub struct FileDiff {
    pub changes: Vec<DiffChange>,
}
