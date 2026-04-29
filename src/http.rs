use ext_php_rs::prelude::*;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;
use std::fs::File;
use std::io::Write;


#[php_class]
pub struct HttpClient {
    client: Client,
}

#[php_impl]
impl HttpClient {

    pub fn __construct() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent("RustPHP/1.0")
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .cookie_store(true)
            .build()
            .unwrap();

        Self { client }
    }

    
    pub fn get(&self, url: String) -> String {
        let res = match self.client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return "Xatolik: GET request failed".into(),
        };

        match res.text() {
            Ok(t) => t,
            Err(_) => "Error: response is not read".into(),
        }
    }
    
    pub fn post(&self, url: String, data: HashMap<String, String>) -> String {
        let res = match self.client.post(&url).form(&data).send() {
            Ok(r) => r,
            Err(_) => return "Error: POST request failed".into(),
        };

        match res.text() {
            Ok(t) => t,
            Err(_) => "Error: response o‘qilmadi".into(),
        }
    }

    pub fn post_with_headers(
        &self,
        url: String,
        data: HashMap<String, String>,
        headers: HashMap<String, String>,
    ) -> String {
        let mut req = self.client.post(&url).form(&data);

        for (k, v) in headers {
            req = req.header(k, v);
        }

        let res = match req.send() {
            Ok(r) => r,
            Err(_) => return "Error: header POST failed".into(),
        };

        match res.text() {
            Ok(t) => t,
            Err(_) => "Error: response is not read".into(),
        }
    }
    
    pub fn get_with_headers(
    &self,
    url: String,
    headers: HashMap<String, String>,
) -> String {
    let mut req = self.client.get(&url);

    for (k, v) in headers {
        req = req.header(k, v);
    }

    let res = match req.send() {
        Ok(r) => r,
        Err(_) => return "Error: GET header failed".into(),
    };

    res.text().unwrap_or("Error: not read".into())
}

pub fn status(&self, url: String) -> i32 {
    match self.client.get(&url).send() {
        Ok(r) => r.status().as_u16() as i32,
        Err(_) => 0,
    }
}

pub fn get_json(&self, url: String) -> String {
    let res = match self.client.get(&url).send() {
        Ok(r) => r,
        Err(_) => return "Error".into(),
    };

    match res.json::<serde_json::Value>() {
        Ok(json) => json.to_string(),
        Err(_) => "JSON parse error".into(),
    }
}

pub fn get_with_query(
    &self,
    url: String,
    params: HashMap<String, String>,
) -> String {
    let res = match self.client.get(&url).query(&params).send() {
        Ok(r) => r,
        Err(_) => return "Error".into(),
    };

    res.text().unwrap_or("Error".into())
}

    pub fn download(&self, url: String, path: String) -> String {
        let res = match self.client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return "Xatolik: download failed".into(),
        };

        let bytes = match res.bytes() {
            Ok(b) => b,
            Err(_) => return "Error: bytes are not read".into(),
        };

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(_) => return "Error: file is not created".into(),
        };

        if file.write_all(&bytes).is_err() {
            return "Error: could not be written".into();
        }

        format!("OK: {}", path)
    }
    
    pub fn upload_file(
    &self,
    url: String,
    file_path: String,
) -> String {
    use reqwest::blocking::multipart;

    let form = match multipart::Form::new().file("file", file_path) {
        Ok(f) => f,
        Err(_) => return "File error".into(),
    };

    let res = match self.client.post(&url).multipart(form).send() {
        Ok(r) => r,
        Err(_) => return "Upload failed".into(),
    };

    res.text().unwrap_or("Error".into())
}

pub fn set_cookies(&self, url: String, cookies: HashMap<String, String>) -> String {
    let mut cookie_string = String::new();

    for (k, v) in cookies {
        cookie_string.push_str(&format!("{}={}; ", k, v));
    }

    let res = match self.client
        .get(&url)
        .header("Cookie", cookie_string)
        .send()
    {
        Ok(r) => r,
        Err(_) => return "Error".into(),
    };

    res.text().unwrap_or("OK".into())
}

}