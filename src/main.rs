use clap::Parser;
use reqwest;
use serde_json::Value;

// #[derive(Parser,Default,Debug, Clone)]
// struct Header {
//     name: String,
//     value: String,
// }

#[derive(Parser,Default,Debug)]
struct Options {
    url: String,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let response = reqwest::get(options.url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let object: Value = serde_json::from_str(&response).unwrap();
    let pretty_json = serde_json::to_string_pretty(&object).unwrap();
    println!("{}", pretty_json);
}
