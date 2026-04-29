use ext_php_rs::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;


#[php_class]
pub struct FileEngine;

#[php_impl]
impl FileEngine {

pub fn write(path: String, content: String) -> bool {
    match File::create(&path) {
        Ok(mut file) => file.write_all(content.as_bytes()).is_ok(),
        Err(_) => false,
    }
}

pub fn read(path: String) -> String {
    fs::read_to_string(&path).unwrap_or_default()
}

pub fn delete(path: String) -> bool {
    fs::remove_file(&path).is_ok()
}

pub fn append(path: String, content: String) -> bool {
    match OpenOptions::new()
        .create(true)  
        .append(true) 
        .open(&path) 
    {
        Ok(mut file) => file.write_all(content.as_bytes()).is_ok(),
        Err(_) => false,
    }
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn unlink(path: &str) -> PhpResult<bool> {
    fs::remove_file(path)
        .map_err(|e| PhpException::from(format!("unlink error: {}", e)))?;
    Ok(true)
}

pub fn copy(from: &str, to: &str) -> PhpResult<bool> {
    let mut src = File::open(from)
        .map_err(|e| PhpException::from(format!("copy open error: {}", e)))?;

    let mut data = Vec::new();
    src.read_to_end(&mut data)
        .map_err(|e| PhpException::from(format!("copy read error: {}", e)))?;

    let mut dest = File::create(to)
        .map_err(|e| PhpException::from(format!("copy create error: {}", e)))?;

    dest.write_all(&data)
        .map_err(|e| PhpException::from(format!("copy write error: {}", e)))?;

    Ok(true)
}

pub fn rename(old: &str, new: &str) -> PhpResult<bool> {
    fs::rename(old, new)
        .map_err(|e| PhpException::from(format!("rename error: {}", e)))?;
    Ok(true)
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn mkdir(path: &str) -> PhpResult<bool> {
    fs::create_dir(path)
        .map_err(|e| PhpException::from(format!("mkdir error: {}", e)))?;
    Ok(true)
}

pub fn rmdir(path: &str) -> PhpResult<bool> {
    fs::remove_dir(path)
        .map_err(|e| PhpException::from(format!("rmdir error: {}", e)))?;
    Ok(true)
}

pub fn scandir(path: Option<String>) -> PhpResult<Vec<String>> {
    let mut files = Vec::new();
    let path = path.unwrap_or_else(|| ".".to_string());

    let entries = fs::read_dir(path)
        .map_err(|e| PhpException::from(format!("scandir error: {}", e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| PhpException::from(format!("{}", e)))?;
        let name = entry.file_name().to_string_lossy().to_string();
        files.push(name);
    }

    Ok(files)
}

}
