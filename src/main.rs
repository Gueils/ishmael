extern crate ishmael;

use ishmael::Github;

fn main() {
    let github = Github::new();
    let user = github.users.get("thelastinuit");
    let repositories = github.repositories.by_user("thelastinuit");

    println!("Login: {:?}", user.login);
    println!("Email: {:?}", user.email);
    println!("url: {:?}", user.url);

    for repo in repositories.iter() {
        println!("Ishmael will dockerize the hell out of these open source projects: {:?} FTFW!!", repo.name);
    }
}
