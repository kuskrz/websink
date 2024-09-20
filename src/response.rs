use colored::Colorize;
use serde::Deserialize;
use std::fs;
use toml::Table;

#[derive(Deserialize, Clone)]
struct ResponsePath {
    path: String,
    body: String,
    headers: Table
}

#[derive(Deserialize, Clone)]
struct ResponseTOMLFile {
    body: String,
    headers: Table,
    paths: Vec<ResponsePath>
}

pub struct ResponseTOML {
    response: Option<ResponseTOMLFile>
}

impl ResponseTOML {
    pub fn parse_response(response_file: &str) -> Self {
        if let Ok(response) = fs::read_to_string(response_file) {
            let response_str = &response[..];
            match toml::from_str::<ResponseTOMLFile>(response_str) {
                Ok(response_toml) => {
                    return ResponseTOML {response: Some(response_toml)};
                }
                Err(e) => {
                    println!("{}: Cannot parse {} : {}", "ERROR".red(), response_file, e);
                    return ResponseTOML {response: None};
                }
            }
        }
        ResponseTOML {response: None}
    }
}
