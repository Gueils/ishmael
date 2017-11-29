use ishmael::client::Client;
use ishmael::github::Github;

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
    pub github: Github,
}

impl Search {
    pub fn new() -> Search {
        let client = Client;

        Search {
            github: Github::new(client),
        }
    }
}
