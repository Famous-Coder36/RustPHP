use ext_php_rs::prelude::*;
use ext_php_rs::zend::ProcessGlobals;
use ext_php_rs::zend::SapiGlobals;
use ext_php_rs::zend::ExecutorGlobals;
use ext_php_rs::types::Zval;


#[php_class]
pub struct Request {
}

#[php_impl]
impl Request {

    pub fn get(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_get_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())
}

pub fn post(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_post_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())
}

pub fn request(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_request_vars()?  
        .get(key.as_str())
        .and_then(|zval: &Zval| zval.string())
        .map(|s| s.to_string())
}

pub fn cookie(name: String) -> Option<String> {
    ProcessGlobals::get()
        .http_cookie_vars()
        .get(name.as_str())
        .and_then(|z| z.string())
}

pub fn server(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_server_vars()?
        .get(key.as_str())
        .and_then(|zval| zval.string())                  
}

pub fn env(key: String) -> Option<String> {
    ProcessGlobals::get()
        .http_env_vars()
        .get(key.as_str())
        .and_then(|zval| zval.string())                  
}

pub fn file(field: String, key: String) -> Option<String> {
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

}
