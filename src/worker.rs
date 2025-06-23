use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::sync::mpsc::Sender;

use crate::stats::FileStats;

// Новая функция: обрабатывает один файл
pub fn process_file(path: PathBuf, stats_tx: &Sender<FileStats>) {
    if let Ok(file) = File::open(&path) {
        let reader = BufReader::new(file);
        let mut lines = 0;
        let mut words = 0;
        let mut bytes = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                lines += 1;
                words += line.split_whitespace().count();
                bytes += line.len();
            }
        }

        let stats = FileStats {
            path,
            lines,
            words,
            bytes,
        };

        let _ = stats_tx.send(stats); // Отправляем результат
    }
}
