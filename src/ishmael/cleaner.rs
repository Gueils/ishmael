use std::process::Command;

/// Ishmael::Cleaner
/// Allows to clear /repos directory.
///
/// Usage:
/// ```
/// ```
pub struct Cleaner<'a> {
    name: &'a String,
}

impl<'a> Cleaner<'a> {
    pub fn new(name: &String) -> Cleaner {
        Cleaner {
            name: name
        }
    }

    pub fn process(&self) -> bool {
        let dir = format!("/repos/{}", &self.name);

        let output = Command::new("rm")
            .current_dir("/")
            .args(&[
                 "-R",
                 dir.as_str()
            ])
            .output()
            .expect("God damn! We couldn't removed the repo directory");

        output.status.success()
    }
}
