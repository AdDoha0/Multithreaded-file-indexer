mod walker;
mod worker;
mod stats;

use core::num;
use std::sync::mpsc::channel;
use std::thread;
use std::path::PathBuf;

// use crate::walker::walk_dir;
// use crate::worker::start_worker;

// use crate::walker::walk_dir;
// use crate::stats::FileStats;


// fn main() {
//     let (file_tx, file_rx) = channel();
//     let  (stats_tx, stats_rx) = channel(); 


//     let root_patch = std::env::args()
//         .nth(1)
//         .expect("Укажите путь к директории"); 
//     let root_path = PathBuf::from(root_patch);


//     let walker_tx = file_tx.clone();
//     let walker = thread::spawn(move || {
//         walk_dir(root_patch, walker_tx);
//     });


//     let num_workers = 4;
//     for _ in 0..num_workers {
//         let rx = file_rx.clone();
//         let tx = stats_tx.clone();
//         thread::spawn(move || {
//             start_worker(rx, tx)
//         })
//     } 


    



// }