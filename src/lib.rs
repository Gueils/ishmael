#![crate_name = "ishmael"]
#![crate_type = "rlib"]

extern crate curl;
extern crate rustc_serialize;

mod repository;
mod query;
mod search;
mod forker;
mod cloner;
mod analyzer;
mod pull_requester;
mod cleaner;
mod client;
mod github;

pub use repository::Repository;
pub use query::Query;
pub use search::Search;
pub use forker::Forker;
pub use cloner::Cloner;
pub use analyzer::Analyzer;
pub use pull_requester::PullRequester;
pub use cleaner::Cleaner;
