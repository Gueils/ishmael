#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;

pub struct RepositoryClient {
    client: Client,
}

impl RepositoryClient {

    pub fn new(c: Client) -> RepositoryClient {
        RepositoryClient { client: c }
    }

    pub fn by_user(self, username: &str) -> Vec<Repository> {
        let mut url = "https://api.github.com/users/".to_string();
        url.push_str(username);
        url.push_str("/repos");

        let req = self.client.request(url.as_ref());

        let repos: Vec<Repository> = json::decode(req.as_ref()).unwrap();

        return repos;
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub url: String,
    pub html_url: String,
    pub clone_url: String,
    pub git_url: String,
    pub languages_url: String,
}
