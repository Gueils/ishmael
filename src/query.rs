use repository::Repository;

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct Query {
    pub items: Vec<Repository>,
}
