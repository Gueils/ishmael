// This was stolen from https://github.com/davidrhyswhite/rust-github
// it was failing, though

#![crate_name = "ishmael"]
#![crate_type = "rlib"]

extern crate curl;
extern crate rustc_serialize;

use client::Client;
use users::UserClient;
use repositories::RepositoryClient;

pub struct Github {
    pub users: UserClient,
    pub repositories: RepositoryClient,
}

impl Github {
    pub fn new() -> Github {
        let client = Client;

        Github {
            users: UserClient::new(client),
            repositories: RepositoryClient::new(client),
        }
    }
}

pub mod client;
pub mod users;
pub mod repositories;
