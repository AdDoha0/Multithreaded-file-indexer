

use std::os::linux::raw::stat;
use std::path::PathBuf;                 // Работа с путями
use std::sync::mpsc::{Receiver, Sender}; // Каналы для получения и отправки
use std::fs::File;                      // Для открытия файлов
use std::io::{BufReader, BufRead};      // Буферизированное чтение строк

use crate::stats::FileStats;    




pub fn start_worker(rx: Receiver<PathBuf>, stats_tx: Sender<FileStats>) {
    for path in rx.iter() {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            let mut lines = 0;
            let mut words = 0;
            let mut bytes = 0;

            for line in reader.lines() {
                if let Ok(line) = line {
                    lines += 1; 
                    words += line.split_ascii_whitespace().count();
                    bytes += line.len();
                }
            } 

            let stats = FileStats {
                path,
                lines, 
                words, 
                bytes
            };
            
            let _ = stats_tx.send(stats);
        };
    }

}