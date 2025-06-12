use crate::indexer::{list_directory_recursion};

mod indexer;



fn main() {
    let path = "/home/user/my_projects/";

    let (total_size, files) = match list_directory_recursion(path) {
        Ok(t) => (t.0, t.1),
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        },
    };

    println!("Total size: {} bytes", total_size);
    println!("Files found: {}", files.len());
    for file in files {
        dbg!(&file);
    }
}