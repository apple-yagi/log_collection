use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content =
        fs::read_to_string("/Users/ryuya/Project/rust_source/log_collection/txt/test.txt")?;
    println!("{}", content);

    let local_datetime = Local::now().format("%Y_%m_%d").to_string();

    let path = "/Users/ryuya/Project/rust_source/log_collection/txt/".to_string();
    let fullpath = path + &local_datetime;
    let mut file = File::create(fullpath)?;
    write!(file, "{}", content)?;
    file.flush()?;

    Ok(())
}
