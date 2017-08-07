#![allow(dead_code)]
use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
const JISHO_API_URL: &'static str = "http://jisho.org/api/v1/search/words?keyword=";

#[derive(Deserialize)]
struct APIResponse {
    meta: BTreeMap<String, Value>,
    data: Vec<SearchResult>,
}

#[derive(Deserialize)]
pub struct SearchResult {
    is_common: Option<bool>,
    tags: Vec<String>,
    pub japanese: Vec<Japanese>,
    pub senses: Vec<Sense>,
    attribution: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
pub struct Japanese {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Deserialize)]
pub struct Sense {
    pub english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
    links: Vec<Link>, 
    tags: Vec<Value>,
    restrictions: Vec<String>,
    source: Vec<BTreeMap<String, Value>>,
    info: Vec<String>,
}

#[derive(Deserialize)]
pub struct Link {
    text: String,
    url: String,
}

pub fn make_request(search_term: &str) -> Vec<SearchResult> {
    let response: APIResponse = reqwest::get(&format!("{}{}", JISHO_API_URL, search_term))
        .expect(&format!("{}: Failed accessing Jisho API", search_term))
        .json()
        .expect(&format!("{}: Failed converting API as JSON", search_term));
    response.data
}

