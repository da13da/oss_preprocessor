#[derive(Debug)]
pub struct FileDiff {
    pub changes: Vec<DiffChange>,
}

#[derive(Debug)]
pub struct DiffChange {
    pub added_code: String,
    pub removed_code: String,
}
