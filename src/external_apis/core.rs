use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenAlexResponse {
    pub meta: Meta,
    pub results: Vec<Work>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AuthorShip {
    pub author: Option<Author>
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub id: Option<String>,
    pub display_name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct KeyWord {
    pub id: Option<String>,
    pub display_name: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Work {
    pub id: String,
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub authorships: Option<Vec<AuthorShip>>,
    pub keywords: Option<Vec<KeyWord>>,
    pub referenced_works: Option<Vec<String>>
}


pub enum ApiSource {
    OpenAlex
}

pub enum ApiGraphType {
    Coauthorship,
    KeywordCooccurrence,
    Cocitation,
}
