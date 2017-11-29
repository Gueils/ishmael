use ishmael::client::Client;
use ishmael::github::Github;

/// Ishmael::Forker
/// Allows to fork a repository.
///
/// Usage:
/// ```
/// let forker = Forker::new();
///
/// forker.github.repo("full_name_repo");
/// ```
pub struct Forker {
    pub github: Github,
}

impl Forker {
    pub fn new() -> Forker {
        let client = Client;

        Forker {
            github: Github::new(client),
        }
    }
}
