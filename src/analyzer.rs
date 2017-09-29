use std::process::Command;

/// Ishmael::Analyzer
/// Allows to call whales to analyze current repository in /repos directory.
///
/// Usage:
/// ```
/// let analyzer = Analyzer::new(repo_name);
/// analyzer.process();
/// ```
pub struct Analyzer<'a> {
    name: &'a String,
}

impl<'a> Analyzer<'a> {
    pub fn new(name: &String) -> Analyzer {
        Analyzer {
            name: name
        }
    }

    pub fn process(&self) -> bool {
        let current_dir = format!("/repos/{}", &self.name);
        let belugas_env = format!("BELUGAS_CODE={}", &current_dir);
        let volume = format!("{}:/code", &current_dir);
        let output = Command::new("docker")
            .current_dir(&current_dir)
            .args(&[
                  "run",
                  "--tty",
                  "--rm",
                  "--env", 
                  "API_BASE_URI=whales.herokuapp.com",
                  "--env", 
                  belugas_env.as_str(),
                  "--volume", 
                  "/var/run/docker.sock:/var/run/docker.sock",
                  "--volume", 
                  volume.as_str(), 
                  "gueils/whales:development",
            ])
            .output()
            .expect("God damn! We couldn't analyzed the repo.");
        println!("{:?}", output);
        output.status.success()
    }
}
