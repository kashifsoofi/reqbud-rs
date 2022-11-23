use std::str::FromStr;

use clap::Parser;
use reqwest::{self, RequestBuilder, Method};
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
    headers: Option<Vec<Header>>,
    #[clap(short('X'), long("request"))]
    request: Option<String>,
}

fn create_request_builder(request: Option<String>, url: String) -> RequestBuilder {
    let client = reqwest::Client::new();
    if let Some(request) = request {
        let method = Method::from_str(request.to_uppercase().as_str()).expect("Unsupported request method");
        client.request(method, url)
    } else {
        client.get(url)
    }
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let mut request_builder = create_request_builder(options.request, options.url);
    if let Some(headers) = options.headers {
        for h in headers {
            request_builder = request_builder.header(h.name, h.value);
        }
    }

    let response = request_builder
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let response = response.text().await.unwrap();
            let object: Value = serde_json::from_str(&response).unwrap();
            let pretty_json = serde_json::to_string_pretty(&object).unwrap();
            println!("{}", pretty_json);
        }
        _ => {
            println!("{}", response.text().await.unwrap());
        }
    };
}
