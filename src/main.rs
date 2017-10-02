extern crate ishmael;

use ishmael::Search;
use ishmael::Forker;
use ishmael::Cloner;
use ishmael::Analyzer;
use ishmael::PullRequester;
use ishmael::Cleaner;

fn main() {
    if cfg!(target_os = "windows") {
        panic!("Sorry, this OS is not supported yet.");
    }

    let search = Search::new();
    let results = search.github.with("rails");

    for repo in results.iter() {
        let forker = Forker::new();
        println!("Ishmael sees a whale at the horizon, he decides to pick up his harpoon: {:?}", repo.name);
        let forked_repo = forker.github.repo(&repo.full_name);

        let cloner = Cloner::new(&forked_repo.clone_url);
        println!("Ishmael gets closer and closer");
        cloner.dispatch();

        let analyzer = Analyzer::new(&forked_repo.name);
        println!("Ishmael takes his time. He ponders a strategy");
        analyzer.process();

        let pull_requester = PullRequester::new(&forked_repo.name);
        println!("Ishmael attacks!");
        pull_requester.process();

        let body = "{\"title\":\"Docker\",\"body\":\"This is Ishmael the Hunter. A bot that tries to dockerize your project. This is a very very BETA, ay!. In time Ishmael will become smarter.\",\"head\":\"ishmaelthehunter:wip/docker\",\"base\":\"master\"}";
        pull_requester.github.pr(&repo.full_name, body);

        let cleaner = Cleaner::new();
        cleaner.process();
        println!("Ishmael aims bows before the great whale, what a hunt!");

        break;
    }
}
