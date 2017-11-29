#![crate_name = "ishmael"]
#![crate_type = "rlib"]

extern crate curl;
extern crate rustc_serialize;

pub mod ishmael {
    pub mod repository;
    pub mod query;
    pub mod search;
    pub mod forker;
    pub mod cloner;
    pub mod analyzer;
    pub mod pull_requester;
    pub mod cleaner;
    pub mod client;
    pub mod github;
}

pub use ishmael::search;
pub use ishmael::forker;
pub use ishmael::cloner;
pub use ishmael::analyzer;
pub use ishmael::pull_requester;
pub use ishmael::cleaner;
