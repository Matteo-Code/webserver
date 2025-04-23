use std::fs;

pub fn get_raw_file_contents(file_path:String) -> String{
    println!("{}",file_path);
    // TODO: Fix bad implementation
    let contents = fs::read_to_string(file_path.replace("/", ""))
        .expect("Should have been able to read the file");

    contents
}