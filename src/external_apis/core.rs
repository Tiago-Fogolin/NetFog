use serde::{Deserialize, Serialize};
use pyo3::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenAlexResponse {
    pub meta: Meta,
    pub results: Vec<Work>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Meta {
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthorShip {
    pub author: Option<Author>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Author {
    pub id: Option<String>,
    pub display_name: Option<String>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthorReponse {
    pub results: Option<Vec<Author>>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct KeyWord {
    pub id: Option<String>,
    pub display_name: Option<String>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct KeyWordResponse {
    pub results: Option<Vec<KeyWord>>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Work {
    pub id: String,
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub authorships: Option<Vec<AuthorShip>>,
    pub keywords: Option<Vec<KeyWord>>,
    pub referenced_works: Option<Vec<String>>
}


#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum OpenAlexGraphType {
    Coauthorship,
    KeywordCooccurrence,
    WorkCocitation,
    AuthorCocitation,
}
