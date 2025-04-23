use std::fs;

pub fn get_raw_file_bytes(file_path:&String) -> Vec<u8>{
    let mut new_path = file_path.clone();
    
    println!("{}",file_path);
    if new_path.starts_with("/") {
        new_path.replace_range(..1, "");
    } 
    let contents = fs::read(new_path)
        .expect("Should have been able to read the file");

    contents
}

pub fn get_raw_file_contents(file_path:&String) -> String {
    let mut new_path = file_path.clone();
    
    println!("{}",file_path);
    if new_path.starts_with("/") {
        new_path.replace_range(..1, "");
    } 
    let contents = fs::read_to_string(new_path)
        .expect("Should have been able to read the file");

    contents
}