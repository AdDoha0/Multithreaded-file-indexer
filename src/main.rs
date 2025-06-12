use crate::indexer::{entry_processing, list_directory};

mod indexer;



fn main() {
    println!("Hello, world!");
    
    match list_directory("/home/user/my_projects") {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
        
}