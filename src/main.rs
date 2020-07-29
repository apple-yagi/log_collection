use chrono::Local;
use dotenv::dotenv;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};

// ファイルを読み込む関数
pub fn cat(path: &String) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;

    match content.len() {
        0 => Err(Error::new(ErrorKind::Other, "Log is empty")),
        _ => Ok(content),
    }
}

// ファイルを書き込む関数
pub fn write(dirpath: String, content: String) -> Result<(), io::Error> {
    // ファイル名は現在の日付
    let filename = Local::now().format("%Y_%m_%d").to_string();

    let fullpath = dirpath + &filename;
    let mut file = File::create(fullpath)?;
    write!(file, "{}", content)?;

    // 書き込みのエラーハンドリング
    file.flush()?;
    Ok(())
}

// 一連のファイル操作
pub fn log_collection(log_path: String, out_path: String) -> Result<(), io::Error> {
    // ログを読み込む
    let content = cat(&log_path)?;

    // ログを別ファイルに書き込む
    write(out_path, content)?;

    // ログを削除
    fs::remove_file(&log_path)?;

    // 空のログファイルを生成
    let mut f = File::create(&log_path)?;
    f.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let access_log_path = env::var("ACCESS_LOG_PATH").expect("ACCESS_LOG_PATH is not defined");
    let access_dir_path = env::var("ACCESS_DIR_PATH").expect("ACCESS_DIR_PATH is not defined");
    let error_log_path = env::var("ERROR_LOG_PATH").expect("ACCESS_LOG_PATH is not defined");
    let error_dir_path = env::var("ERROR_DIR_PATH").expect("ERROR_DIR_PATH is not defined");

    match log_collection(access_log_path, access_dir_path) {
        Ok(()) => println!("Success!!"),
        Err(err) => println!("failure: {}", err),
    }

    match log_collection(error_log_path, error_dir_path) {
        Ok(()) => println!("Success!!"),
        Err(err) => println!("failure: {}", err),
    }

    Ok(())
}
