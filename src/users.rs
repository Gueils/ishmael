#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;

pub struct UserClient {
    client: Client,
}

impl UserClient {

    pub fn new(c: Client) -> UserClient {
        UserClient { client: c }
    }

    pub fn get(self, name: &str) -> User {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(name);

        let req = self.client.request(url.as_ref());

        let user: User = json::decode(req.as_ref()).unwrap();

        return user;
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct User {
    pub login: String,
    pub id: i32,
    pub url: String,
    pub html_url: String,
    pub email: Option<String>,
}
