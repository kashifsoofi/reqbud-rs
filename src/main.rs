use clap::Parser;
use reqwest;
use serde_json::Value;

#[derive(Parser,Default,Debug, Clone)]
struct Header {
    name: String,
    value: String,
}

fn header_parser(s: &str) -> Result<Header, String> {
    let parts: Vec<&str> = s.split(":").collect();
    if parts.len() != 2 {
        return Err(String::from("Invalid header format, must be name:value"));
    }

    Ok(Header{
        name: String::from(parts[0]),
        value: String::from(parts[1]),
    })
}

#[derive(Parser,Default,Debug)]
struct Options {
    url: String,
    #[clap(short('H'), long("header"), value_parser = header_parser)]
    header: Option<Vec<Header>>,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let client = reqwest::Client::new();
    let mut request_builder = client
        .get(options.url);
    if let Some(headers) = options.header {
        for h in headers {
            request_builder = request_builder.header(h.name, h.value);
        }
    }

    let response = request_builder
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let object: Value = serde_json::from_str(&response).unwrap();
    let pretty_json = serde_json::to_string_pretty(&object).unwrap();
    println!("{}", pretty_json);
}
