use std::path::Path;
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


pub fn list_directory_recursion<P: AsRef<Path>>(path: P) -> std::io::Result<(u64, Vec<FileInfo>)> {
    let mut total_size = 0;
    let mut collected_info:Vec<FileInfo>  = Vec::new();

    for entry_result in read_dir(path)? {
        let entry = entry_result?;
        let metadata = entry.metadata()?; 

        if metadata.is_file() {
            total_size += metadata.len()
        } else if metadata.is_dir() {
            let (dir_size, mut dir_info) = list_directory_recursion(entry.path())?;
            total_size += dir_size;
            collected_info.append(&mut dir_info);
        }
        let file_info = entry_processing(&entry)?;
        collected_info.push(file_info);
    }

    Ok((total_size, collected_info))

    
}  