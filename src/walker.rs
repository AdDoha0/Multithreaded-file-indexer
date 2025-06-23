

use std::fs::read_dir;          
use std::path::PathBuf;          
use std::sync::mpsc::Sender;

pub fn walk_dir(path: PathBuf, tx: Sender<PathBuf>) {
    if let Ok(entries) = read_dir(path) {
        for entry in  entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_dir(path, tx.clone());
            } else if path.is_file() {
                let _ = tx.send(path);
            }
        };
    };
}