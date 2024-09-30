use colored::Colorize;
use serde::Deserialize;
use std::fs;
use toml::Table;

#[derive(Deserialize, Clone)]
struct ResponsePath {
    path: String,
    method: String,
    body: String,
    headers: Table,
}

#[derive(Deserialize, Clone)]
struct ResponseTOMLFile {
    body: String,
    headers: Table,
    paths: Vec<ResponsePath>,
}

#[derive(Clone)]
pub struct ResponseTOML {
    response: Option<ResponseTOMLFile>,
}

impl ResponseTOML {
    pub fn new_empty() -> Self {
        ResponseTOML { response: None }
    }

    pub fn parse_response(response_file: &str) -> Self {
        if let Ok(response) = fs::read_to_string(response_file) {
            let response_str = &response[..];
            match toml::from_str::<ResponseTOMLFile>(response_str) {
                Ok(response_toml) => {
                    return ResponseTOML {
                        response: Some(response_toml),
                    };
                }
                Err(e) => {
                    println!("{}: Cannot parse {} : {}", "ERROR".red(), response_file, e);
                    return ResponseTOML { response: None };
                }
            }
        }
        ResponseTOML { response: None }
    }

    pub fn get_body(&self, path: &str, meth: &str) -> Option<String> {
        match &self.response {
            Some(toml) => {
                let mut body = toml.body.clone();
                for toml_path in &toml.paths {
                    if toml_path.method.to_uppercase() == meth.to_uppercase()
                        && path.contains(&toml_path.path)
                    {
                        body = toml_path.body.clone();
                        break;
                    }
                }
                Some(body)
            }
            None => None,
        }
    }

    pub fn get_headers(&self, path: &str, meth: &str) -> Option<Vec<(String, String)>> {
        match &self.response {
            Some(toml) => {
                let mut headers = toml
                    .headers
                    .clone()
                    .into_iter()
                    .map(|x| (x.0, String::from(x.1.as_str().unwrap_or_default())))
                    .collect();
                for toml_path in &toml.paths {
                    if toml_path.method.to_uppercase() == meth.to_uppercase()
                        && path.contains(&toml_path.path)
                    {
                        headers = toml_path
                            .headers
                            .clone()
                            .into_iter()
                            .map(|x| (x.0, String::from(x.1.as_str().unwrap_or_default())))
                            .collect();
                        break;
                    }
                }
                Some(headers)
            }
            None => None,
        }
    }
}

/*
if let Some(h) = response_headers_toml {
                responseh = h
                    .into_iter()
                    .map(|x| (x.0, String::from(x.1.as_str().unwrap_or_default())))
                    .collect();
            }
*/
