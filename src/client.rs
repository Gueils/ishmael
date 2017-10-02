extern crate curl;

use std::str;
use curl::http;
use std::collections::HashMap;

#[derive(Copy,Clone)]
pub struct Client;

impl Client {
    pub fn get(self, url: &str) -> String {
        let res = http::handle()
            .get(url)
            .header("User-Agent", "Ishmael-Github-Client")
            .exec().unwrap();
        let body = match str::from_utf8(res.get_body()) {
            Ok(b) => b,
            Err(..) => "Unable to parse"
        };

        return body.to_string();
    }

    pub fn post(self, url: &str, body: &str) -> String {
        let token = format!("token {}", env!("GITHUB_API_KEY"));
        let mut headers = HashMap::new();
        headers.insert("User-Agent", "Ishmael-Github-Client");
        headers.insert("Authorization", token.as_str());

        let res = http::handle()
            .post(url, body)
            .headers(headers.into_iter())
            .exec().unwrap();
        let body = match str::from_utf8(res.get_body()) {
            Ok(b) => b,
            Err(..) => "Unable to parse"
        };

        return body.to_string();
    }
}
