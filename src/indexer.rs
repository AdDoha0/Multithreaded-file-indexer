use std::{path::PathBuf, time::SystemTime};
use std::fs::{read_dir, DirEntry};



// Важные методы DirEntry
//     file_name(): Возвращает имя файла/директории (OsString).
//     path(): Возвращает полный путь к записи (PathBuf).
//     file_type(): Возвращает тип записи (io::Result<FileType>), где можно узнать, является ли запись файлом, директорией или символической ссылкой.
//     metadata(): Возвращает метаданные файла (io::Result<Metadata>), такие как размер, права доступа и время модификации.

#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,
    size: u64, 
    extension: Option<String>,
    modified: SystemTime,
}

pub fn entry_processing(entry: &DirEntry) -> std::io::Result<FileInfo> {

    let path = entry.path();
    
    let metadata = entry.metadata()?;
    let size = metadata.len();
    let modified = metadata.modified()?;


    let extension = if entry.file_type()?.is_file() {
        path.extension()
            .and_then(|ext| ext.to_str().map(|s| s.to_string()))
    } else {
        None
    };

    let file_info = FileInfo {
        path,
        size,
        extension,
        modified,
    };

    Ok(file_info)

}


pub fn list_directory(path: &str) -> std::io::Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let file_info = entry_processing(&entry)?; 
        dbg!(&file_info);
    }

    Ok(())
}
