extern crate ishmael;

use ishmael::Github;
use std::process::Command;

fn main() {
  let github = Github::new();
  let repositories = github.repositories.search("rails");

  for repo in repositories.iter() {
    println!("Ishmael will dockerize the hell out of these open source projects: {:?} FTFW!!", repo.name);

    let output = Command::new("git")
      .arg("clone")
      .arg(&repo.clone_url)
      .status()
      .expect("didn't work.");

    println!("output: {}", output);
    break;
  }
}
