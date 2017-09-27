// This was stolen from https://github.com/davidrhyswhite/rust-github
// it was failing, though
#![crate_name = "ishmael"]
#![crate_type = "rlib"]

extern crate curl;
extern crate rustc_serialize;

use client::Client;
use repositories::RepositoryClient;
use std::process::Command;

/// Ishmael::Search
/// Allows to search repositories.
///
/// Usage:
/// ```
/// let search = Search::new();
/// let results = search.repositories.where("keyword");
///
/// for repository in results.iter() {
///   ...
/// }
/// ```
pub struct Search {
    pub repositories: RepositoryClient,
}

impl Search {
    pub fn new() -> Search {
        let client = Client;

        Search {
            repositories: RepositoryClient::new(client),
        }
    }
}

/// Ishmael::Forker
/// Allows to fork a repository.
///
/// Usage:
/// ```
/// ```
pub struct Forker<'a> {
    repo_url: &'a String,
}

impl<'a> Forker<'a> {
    pub fn new(repo_url: &String) -> Forker {
        Forker {
            repo_url: repo_url,
        }
    }

    pub fn process(&self) -> &String {
        return self.repo_url;
    }
}

/// Ishmael::Cloner
/// Allows to clone a repository in the /repos directory.
///
/// Usage:
/// ```
/// let cloner = Cloner::new(&url);
///
/// cloner.dispatch()
/// ```
pub struct Cloner<'a> {
    repo_url: &'a String,
}

impl<'a> Cloner<'a> {
    pub fn new(repo_url: &String) -> Cloner {
        Cloner {
            repo_url: repo_url,
        }
    }

    pub fn dispatch(&self) -> bool {
        self.create_repos_folder() && self.clone()
    }

    fn clone(&self) -> bool {
        let output = Command::new("git")
            .current_dir("/repos")
            .args(&["clone", &self.repo_url])
            .output()
            .expect("God damn! We couldn't clone the repo.");

        output.status.success()
    }

    fn create_repos_folder(&self) -> bool {
        let output = Command::new("mkdir")
            .args(&["-p", "/repos"])
            .output()
            .expect("God damn! We couldn't created the directory where we are going to clone repos.");

        output.status.success()
    }
}

/// Ishmael::Analyzer
/// Allows to call whales to analyze current repository in /repos directory.
///
/// Usage:
/// ```
/// ```
pub struct Analyzer {}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer {}
    }

    pub fn process(&self) {}
}

/// Ishmael::PullRequester
/// Allows to make a pull request.
///
/// Usage:
/// ```
/// ```
pub struct PullRequester {}

impl PullRequester {
    pub fn new() -> PullRequester {
        PullRequester {}
    }

    pub fn process(&self) {}
}

/// Ishmael::Cleaner
/// Allows to clear /repos directory.
///
/// Usage:
/// ```
/// ```
pub struct Cleaner {}

impl Cleaner {
    pub fn new() -> Cleaner {
        Cleaner {}
    }

    pub fn process(&self) {}
}


pub mod client;
pub mod repositories;
