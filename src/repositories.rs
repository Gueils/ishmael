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

    pub fn search(self, keyword: &str) ->Vec<Repository> {
        let mut url = "https://api.github.com/search/repositories?utf8=%E2%9C%93&q=topic%3A".to_string();
        url.push_str(keyword);
        url.push_str("&type=Repositories");

        let req = self.client.request(url.as_ref());
        
        let query: Query = json::decode(req.as_ref()).unwrap();
        let repos: Vec<Repository> = query.items;

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

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct Query {
    pub items: Vec<Repository>,
}
