use crate::RequestConfig;
use std::fs;

use axum::body;
use axum::extract::Request;
use axum::extract::State;

pub async fn fullp(State(req_cfg): State<RequestConfig>, request: Request) -> String {
    if !req_cfg.noout {
        println!("========== HEADER ==========");
        for (key, val) in request.headers().into_iter() {
            println!("{}:{:?}", key, val);
        }

        println!("=========== BODY ===========");
        let body = body::to_bytes(request.into_body(), req_cfg.bytes)
            .await
            .unwrap();
        println!("{:?}", body);
    }

    let mut return_string = String::from("");
    if let Some(file_name) = req_cfg.response {
        if let Ok(file_content) = fs::read_to_string(file_name.to_owned()) {
            return_string = file_content;
        }
    }

    return_string
}

pub async fn fullg(State(req_cfg): State<RequestConfig>) -> String {
    let mut return_string = String::from("");
    if let Some(file_name) = req_cfg.response {
        if let Ok(file_content) = fs::read_to_string(file_name.to_owned()) {
            return_string = file_content;
        }
    }

    return_string
}

pub async fn empty() {}
