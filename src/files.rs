use tauri::command;
use std::{fs, thread};
use std::path::Path;

#[command]
pub fn get_files(directory:String) -> Vec<String> {
    let mut files: Vec<String> = fs::read_dir(directory).unwrap().map(|res| res.unwrap().path().to_string_lossy().to_string()).collect();
    files.sort();
    files
}

#[command]
pub fn home() -> String {
    std::env::home_dir().unwrap().as_path().to_string_lossy().to_string()
}

#[command]
pub fn remove_hidden(files:Vec<String>) -> Vec<String> {
    files.into_iter().filter(|x| !x.split("/").last().unwrap().split("\\").last().unwrap().starts_with(".")).collect()
}

#[command]
pub fn remove_folders(files:Vec<String>) -> Vec<String> {
    files.into_iter().filter(|x| !Path::new(x).is_dir()).collect()
}

#[command]
pub fn remove_files(files:Vec<String>) -> Vec<String> {
    files.into_iter().filter(|x| Path::new(x).is_dir()).collect()
}

#[command]
pub fn is_folder(name:String) -> bool {
    Path::new(&name).is_dir()
}

#[command]
pub fn open(path:String) {
    thread::spawn(|| {
        open::that(path).unwrap();
    });
}