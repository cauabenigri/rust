use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("USER_AGENT", HeaderValue::from_static("reqwest"));
    headers.insert("CONTENT_TYPE", HeaderValue::from_static("image/png"));
    headers
}

fn construct_form() -> HashMap<&'static str, &'static str>{
    let mut params = HashMap::new();
    params.insert("lang", "rust");
    params
}

fn main() {
    println!("Hello, world!");
    let url = "https://google.com";
    let client = reqwest::blocking::Client::new();
    let req = client.get(url).headers(construct_headers()).form(&construct_form()).send();
    print!("{req:?}");
}
