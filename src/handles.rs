use crate::RequestConfig;

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
        let body_result = body::to_bytes(request.into_body(), req_cfg.bytes).await;
        match body_result {
            Ok(body) => println!("{:?}", body),
            Err(e) => println!("{}", e),
        }
    }

    req_cfg.response_body
}

pub async fn fullg(State(req_cfg): State<RequestConfig>, request: Request) -> String {
    if !req_cfg.noout {
        println!("========== HEADER ==========");
        for (key, val) in request.headers().into_iter() {
            println!("{}:{:?}", key, val);
        }
    }

    req_cfg.response_body
}

pub async fn empty() {}
