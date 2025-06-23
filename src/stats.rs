use std::path::PathBuf; 
#[derive(Debug)]
pub struct FileStats {
    pub path: PathBuf,
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}