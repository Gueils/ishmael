#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub url: String,
    pub html_url: String,
    pub clone_url: String,
    pub git_url: String,
    pub languages_url: String,
}
