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
