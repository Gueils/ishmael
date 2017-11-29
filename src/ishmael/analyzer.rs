use std::process::Command;
use std::fs::File;
use std::io::Write;

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
        let dockerfile_path = format!("{}/Dockerfile.beta", &current_dir);
        let dockercompose_path = format!("{}/docker-compose.yml.beta", &current_dir);
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
                  "gueils/whales",
            ])
            .output()
            .expect("God damn! We couldn't analyzed the repo.");

        let content = String::from_utf8(output.stdout).unwrap();
        let dockerfile = content.split("\r\n--------------------------------------------------\r\n").nth(0);
        let dockercompose = content.split("\r\n--------------------------------------------------\r\n").nth(1);
        self.write_file(dockerfile.unwrap(), dockerfile_path.as_str());
        self.write_file(dockercompose.unwrap(), dockercompose_path.as_str());

        output.status.success()
    }

    fn write_file(&self, content: &str, path: &str) {
        let mut f = File::create(path).expect("Unable to create file");
        f.write_all(content.as_bytes()).expect("Unable to write data");
    }
}
