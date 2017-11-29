use std::process::Command;

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
