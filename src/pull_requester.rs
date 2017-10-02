use std::process::Command;
use client::Client;
use github::Github;

/// Ishmael::PullRequester
/// Allows to make a pull request.
///
/// Usage:
/// ```
/// ```
pub struct PullRequester<'a> {
  name: &'a String,
  pub github: Github,
}

impl<'a> PullRequester<'a> {
    pub fn new(name: &String) -> PullRequester {
        let client = Client;
        PullRequester {
            github: Github::new(client),
            name: name
        }
    }

    pub fn process(&self) -> bool {
        let current_dir = format!("/repos/{}", &self.name);
        let url = format!("git@github.com:ishmaelthehunter/{}.git", &self.name);
        Command::new("git")
            .current_dir(&current_dir)
            .args(&[
                  "remote",
                  "set-url",
                  "origin",
                  url.as_str()
            ])
            .output()
            .expect("God damn! We couldn't change git url");
        Command::new("git")
            .current_dir(&current_dir)
            .args(&[
                  "checkout",
                  "-b",
                  "wip/docker"
            ])
            .output()
            .expect("God damn! We couldn't created a branch");
        Command::new("git")
            .current_dir(&current_dir)
            .args(&[
                  "add",
                  "."
            ])
            .output()
            .expect("God damn! We couldn't added the files");
        Command::new("git")
            .current_dir(&current_dir)
            .args(&[
                  "commit",
                  "-m",
                  "\"Adds a beta dockerfile and docker-compose file.\""
            ])
            .output()
            .expect("God damn! We couldn't committed changes");
        let output_push = Command::new("git")
            .current_dir(&current_dir)
            .args(&[
                  "push",
                  "origin",
                  "wip/docker"
            ])
            .output()
            .expect("God damn! We couldn't pushed branch");

        output_push.status.success()
    }
}
