#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;
use repository::Repository;
use query::Query;

pub struct Github {
    client: Client,
}

impl Github {
    pub fn new(client: Client) -> Github {
        Github { client: client }
    }

    pub fn with(self, keyword: &str) -> Vec<Repository> {
        let mut url = "https://api.github.com/search/repositories?utf8=%E2%9C%93&q=topic%3A&".to_string();
        url.push_str(keyword);
        url.push_str("&type=Repositories");

        let req = self.client.get(url.as_ref());

        let query: Query = json::decode(req.as_ref()).unwrap();
        let repos: Vec<Repository> = query.items;

        return repos;
    }

    pub fn repo(self, full_name: &str) -> Repository {
        let mut url = "https://api.github.com/repos/".to_string();
        url.push_str(full_name);
        url.push_str("/forks");

        let req = self.client.post(url.as_ref(), "");
        let repo: Repository = json::decode(req.as_ref()).unwrap();

        return repo;
    }

    pub fn pr(self, full_name: &str, body: &str) {
        let mut url = "https://api.github.com/repos/".to_string();
        url.push_str(full_name);
        url.push_str("/pulls");

        self.client.post(url.as_ref(), body);
    }
}
