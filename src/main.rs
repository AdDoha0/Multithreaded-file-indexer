use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::path::PathBuf;

mod walker;
mod worker;
mod stats;

use crate::worker::start_worker;
use crate::walker::walk_dir;
use crate::stats::FileStats;

fn main() {
    let (file_tx, file_rx) = channel(); // Канал: обходчик → воркеры
    let (stats_tx, stats_rx) = channel(); // Канал: воркеры → главный поток

    let root_path = std::env::args()
        .nth(1)
        .expect("Укажите путь к директории");
    let root_path = PathBuf::from(root_path);

    // Оборачиваем file_rx в Arc<Mutex<...>>
    let shared_file_rx = Arc::new(Mutex::new(file_rx));

    // Запускаем обходчик (он отправляет пути файлов в file_tx)
    let walker_tx = file_tx.clone();
    let walker = thread::spawn(move || {
        walk_dir(root_path, walker_tx);
    });

    // Запускаем воркеры
    let num_workers = 4;
    for _ in 0..num_workers {
        let rx = Arc::clone(&shared_file_rx); // Клонируем Arc (всё ещё один и тот же Receiver)
        let tx = stats_tx.clone(); // Статистика отправляется независимо

        thread::spawn(move || {
            loop {
                // 🔒 Получаем доступ к приёмнику
                let path = {
                    let guard = rx.lock().unwrap(); // Блокируем доступ к Receiver
                    guard.recv() // Получаем путь
                };

                match path {
                    Ok(path) => {
                        worker::process_file(path, &tx); // Обработка одного файла
                    }
                    Err(_) => break, // Канал закрыт — завершаем поток
                }
            }
        });
    }

    drop(file_tx);  // Завершаем отправку путей
    drop(stats_tx); // Завершаем отправку статистики

    walker.join().unwrap(); // Ждём завершения обходчика

    // Собираем общую статистику
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;

    for stat in stats_rx.iter() {
        println!("{:?}", stat);
        total_lines += stat.lines;
        total_words += stat.words;
        total_bytes += stat.bytes;
    }

    println!(
        "Итого: {} строк, {} слов, {} байт",
        total_lines, total_words, total_bytes
    );
}
