#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use ext_php_rs::zend::ProcessGlobals;
use ext_php_rs::zend::SapiGlobals;
use ext_php_rs::zend::ExecutorGlobals;
use ext_php_rs::php_output;
use ext_php_rs::exception::PhpException;
use std::panic;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::io::Read;
use serde_json::Value;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use std::process;
//use tokio::time::{sleep, Duration};

#[php_class]
pub struct RustEngine {
}

 #[php_impl]
impl RustEngine {
    
    pub fn write_file(path: String, content: String) -> bool {
    match File::create(&path) {
        Ok(mut file) => file.write_all(content.as_bytes()).is_ok(),
        Err(_) => false,
    }
}

pub fn read_file(path: String) -> String {
    fs::read_to_string(&path).unwrap_or_default()
}

pub fn delete_file(path: String) -> bool {
    fs::remove_file(&path).is_ok()
}

pub fn append_file(path: String, content: String) -> bool {
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

pub fn get_param(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_get_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())
}

pub fn post_param(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_post_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())
}

pub fn request_param(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_request_vars()?  
        .get(key.as_str())
        .and_then(|zval: &Zval| zval.string())
        .map(|s| s.to_string())
}

pub fn get_cookie(name: String) -> Option<String> {
    ProcessGlobals::get()
        .http_cookie_vars()
        .get(name.as_str())
        .and_then(|z| z.string())
}

pub fn server_var(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_server_vars()?
        .get(key.as_str())
        .and_then(|zval| zval.string())                  
}

pub fn env_param(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_env_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())                  
}

pub fn file_field(field: String, key: String) -> Option<String> {
let globals = ProcessGlobals::get();
    let files = globals.http_files_vars();

    // $_FILES structure: $_FILES['field']['name'], ['tmp_name'], ['size'], etc.
    files
        .get(field.as_str())?
        .array()?
        .get(key.as_str())
        .and_then(|zval| zval.string())	
}

pub fn get_request_info() -> Vec<String> {
    let globals = SapiGlobals::get();
    let request_info = globals.request_info();

    let mut info = Vec::new();

    if let Some(method) = request_info.request_method() {
        info.push(format!("Method: {}", method));
    }
    if let Some(uri) = request_info.request_uri() {
        info.push(format!("URI: {}", uri));
    }
    if let Some(query) = request_info.query_string() {
        info.push(format!("Query: {}", query));
    }
    if let Some(content_type) = request_info.content_type() {
        info.push(format!("Content-Type: {}", content_type));
    }
    info.push(format!("Content-Length: {}", request_info.content_length()));

    info
}


pub fn get_constant(name: String) -> Option<String> {
    let globals = ExecutorGlobals::get();
    globals
        .constants()?
        .get(name.as_str())
        .and_then(|zval| zval.string())
}


pub fn get_ini(key: String) -> Option<String> {
    ExecutorGlobals::get()
        .ini_values()
        .get(key.as_str())
        .cloned()
        .flatten()
}



pub fn to_uppercase(text: String) -> String {
    text.to_uppercase()
}


pub fn slugify(text: String) -> String {
    text.replace(" ", "-").to_lowercase()
}


pub fn parse_json(data: String) -> String {
    let v: Value = serde_json::from_str(&data).unwrap();
    v.to_string()
}


pub fn generate_token() -> String {
    Uuid::new_v4().to_string()
}


pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}


pub fn println(name: &str) {
    php_println!("{}", name);
}


pub fn output(input: &str) {
    if std::path::Path::new(input).exists() {
        let data = std::fs::read(input).unwrap();
        let _ = php_output!(&data);
    } else {
        let _ = php_output!(input.as_bytes());
    }
}

pub fn explode(delimiter: String, text: String) -> Vec<String> {
    text.split(&delimiter)
        .map(|s| s.to_string())
        .collect()
}

pub fn panic(msg: &str) {
    panic!("{}", msg);
}

pub fn ferror(msg: &str) -> PhpResult<()> {
    Err(msg.into())
}

pub fn exc(msg: &str) -> PhpResult<()> {
    Err(PhpException::from(msg))
}

pub fn stop(msg: &str) {
    let _ = php_output!(msg.as_bytes());
    process::exit(1);
}

pub fn var_dump(val: &Zval) {
    php_println!("{:#?}", val);
}

}



#[php_function]
pub fn str_example(input: String) -> String {
    format!("Hello {}", input)
}

#[php_function]
pub fn test_numbers(a: i32, b: u32, c: f32) -> String {
    format!("a {} b {} c {}", a, b, c)
}

#[php_function]
#[php(name = "hi_world")]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}
#[php_function]
pub fn forexm(n: i32) {

    for i in 1..=n {  
        println!("{}", i)
    }
}






#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(hello_world))
        .function(wrap_function!(test_numbers))
        .function(wrap_function!(str_example))
        .function(wrap_function!(forexm))
        .class::<RustEngine>()
}
