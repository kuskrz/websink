use serde::Deserialize;
use std::fs;
use toml::{map::Map, Table, Value};

#[derive(Deserialize)]
struct Response {
    body: String,
    headers: Table,
}

pub fn parse_response(response_file: &str) -> (Option<String>, Option<Map<String, Value>>) {
    if let Ok(response) = fs::read_to_string(response_file) {
        let response_str = &response[..];
        match toml::from_str::<Response>(response_str) {
            Ok(response_toml) => {
                return (Some(response_toml.body), Some(response_toml.headers));
            }
            Err(e) => {
                println!("Cannot parse {} : {}", response_file, e);
                return (None, None);
            }
        }
    }
    (None, None)
}
