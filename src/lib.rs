#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use ext_php_rs::php_output;
use ext_php_rs::exception::PhpException;
use ext_php_rs::types::ArrayKey;
use std::panic;
use serde_json::Value;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use std::process;

//use tokio::time::{sleep, Duration};

mod hash;
mod password;
mod token;
mod jwt;
mod hmac;
mod sign;
mod telegram;
mod http;
mod file;
mod request;
mod mysql;
mod rayon;
mod r#async;

use r#async::{start_workers, push_job};
use rayon::RayonClass;
use mysql::DB;
use request::Request;
use file::FileEngine;
use http::HttpClient;
use telegram::TelegramBot;

#[php_class]
pub struct RustEngine {
}

 #[php_impl]
impl RustEngine {

pub fn r#loop(callback: ZendCallable) {
    loop {
        match callback.try_call(vec![]) {
            Ok(result) => {
                if let Some(b) = result.bool() {
                    if b == false {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
}
		
pub fn r#for(start: i32, end: i32, callback: ZendCallable) {
    if start < end {
        for i in start..=end {
            match callback.try_call(vec![&Zval::from(i)]) {
                Ok(result) => {
                    if let Some(b) = result.bool() {
                        if b == false {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    } else if start > end {
        let mut i = start;
        while i > end {
            match callback.try_call(vec![&Zval::from(i)]) {
                Ok(result) => {
                    if let Some(b) = result.bool() {
                        if b == false {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
            i -= 1;
        }
    } else {
        let _ = callback.try_call(vec![&Zval::from(start)]);
    }
}

pub fn foreach(arr: &Zval, callback: ZendCallable) {
    let array = match arr.array() {
        Some(a) => a,
        None => return,
    };

    for (_key, value) in array.iter() {
        let ret = callback.try_call(vec![<&Zval>::from(value)]);

        match ret {
            Ok(r) => {
                // PHP: return false => break
                if let Some(stop) = r.bool() {
                    if stop == false {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }
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


pub fn println(input: &str) {
    php_println!("{}", input);
}

pub fn print(input: &str) {
    php_print!("{}", input);
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

pub fn mb_stripos(haystack: &str, needle: &str) -> PhpResult<Zval> {
    let hay = haystack.to_lowercase();
    let nee = needle.to_lowercase();

    if let Some(pos) = hay.find(&nee) {
        Ok(Zval::from(pos as i32))
    } else {
        Ok(Zval::from(false))
    }
}

pub fn str_replace(from: &str, to: &str, subject: &str) -> String {
    subject.replace(from, to)
}

pub fn starts_with(haystack: &str, needle: &str) -> bool {
    haystack.starts_with(needle)
}

pub fn strpos(haystack: &str, needle: &str) -> PhpResult<Zval> {
    if let Some(pos) = haystack.find(needle) {
        Ok(Zval::from(pos as i32)) 
    } else {
        Ok(Zval::from(false))
    }
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

pub fn debug(val: &Zval) {
dump_internal(val, 0);
}

}

#[php_class]
pub struct RCrypto;

#[php_impl]
impl RCrypto {

    pub fn sha256(text: String) -> String {
    hash::sha256(text)
}

pub fn blake3(text: String) -> String {
    hash::blake3_hash(text)
}

pub fn password_hash(pass: String) -> String {
    password::hash(pass)
}

pub fn password_verify(pass: String, hash: String) -> bool {
    password::verify(pass, hash)
}

pub fn token() -> String {
    token::generate_token()
}

pub fn jwt_encode(user: String, secret: String) -> String {
    jwt::jwt_encode(user, &secret)
}

pub fn jwt_decode(token: String, secret: String) -> String {
    jwt::jwt_decode(token, &secret)
}

pub fn hmac(data: String, key: String) -> String {
    hmac::hmac_sign(data, key)
}

pub fn hmac_verify(data: String, key: String, hash: String) -> bool {
    hmac::hmac_verify(data, key, hash)
}

pub fn sign_message(msg: String) -> String {
    sign::sign_message(msg)
}

pub fn verify_message(msg: String, data: String) -> bool {
    sign::verify_message(msg, data)
}


}

fn dump_internal(val: &Zval, indent: usize) {
    let pad = " ".repeat(indent);

    if val.is_null() {
        php_println!("{}NULL", pad);
    }
    else if let Some(b) = val.bool() {
        php_println!("{}bool({})", pad, if b { "true" } else { "false" });
    }
    else if let Some(i) = val.long() {
        php_println!("{}int({})", pad, i);
    }
    else if let Some(f) = val.double() {
        php_println!("{}float({})", pad, f);
    }
    else if let Some(s) = val.string() {
        php_println!("{}string({}) \"{}\"", pad, s.len(), s);
    }
    else if let Some(arr) = val.array() {
        php_println!("{}array({}) {{", pad, arr.len());

        for (key, value) in arr.iter() {
            match key {
    ArrayKey::Long(i) => {
        php_print!("  [{}] => ", i);
    }

    ArrayKey::String(s) => {
        php_print!("  [\"{}\"] => ", s);
    }

    _ => {
        php_print!("  [unknown] => ");
    }
}

            dump_internal(value, indent + 4);
        }

        php_println!("{}}}", pad);
    }  else if val.is_object() {
        php_println!("{}object(...) {{", pad);
        php_println!("{}  [object dump not fully implemented]", pad);
        php_println!("{}}}", pad);
    }  else if val.is_resource() {
        php_println!("{}resource({:?})", pad, val.resource());
    } else {
        php_println!("{}unknown type", pad);
    }
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


#[php_function]
pub fn dispatch_job(func: String, data: String, priority: i32) {
    push_job(func, data, priority);
   // "queued".to_string()
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
	start_workers(6);
    module
        .function(wrap_function!(hello_world))
        .function(wrap_function!(forexm))
        .function(wrap_function!(dispatch_job))
        .class::<RustEngine>()
        .class::<RCrypto>()
        .class::<TelegramBot>()
        .class::<HttpClient>()
        .class::<FileEngine>()
        .class::<Request>()
        .class::<DB>()
        .class::<RayonClass>()
}
