use ext_php_rs::prelude::*;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;


#[php_class]
pub struct TelegramBot {
    token: String,
}

#[php_impl]
impl TelegramBot {

    pub fn __construct(token: String) -> Self {
        Self { token }
    }

    pub fn bot(&self, method: String, data: HashMap<String, String>) -> String {
        let url = format!(
            "https://api.telegram.org/bot{}/{}",
            self.token, method
        );

        let client = match Client::builder()
            .timeout(Duration::from_secs(20))
            .user_agent("RustPHP/1.0")
            .build()
        {
            Ok(c) => c,
            Err(_) => return "Client error".into(),
        };

        let response = match client.post(&url).form(&data).send() {
            Ok(res) => res,
            Err(_) => return "Request error".into(),
        };

        match response.text() {
            Ok(text) => text,
            Err(_) => "Response error".into(),
        }
    }

    pub fn send_message(&self, chat_id: String, text: String) -> String {
        let mut data = HashMap::new();
        data.insert("chat_id".to_string(), chat_id);
        data.insert("text".to_string(), text);

        self.bot("sendMessage".to_string(), data)
    }
}