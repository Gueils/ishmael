#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;
use repository::Repository;
use query::Query;

pub struct Github {
    client: Client,
}

impl Github {
    pub fn new(c: Client) -> Github {
        Github { client: c }
    }

    pub fn with(self, keyword: &str) -> Vec<Repository> {
        let mut url = "https://api.github.com/search/repositories?utf8=%E2%9C%93&q=topic%3A".to_string();
        url.push_str(keyword);
        url.push_str("&type=Repositories");

        let req = self.client.request(url.as_ref());

        let query: Query = json::decode(req.as_ref()).unwrap();
        let repos: Vec<Repository> = query.items;

        return repos;
    }
}
