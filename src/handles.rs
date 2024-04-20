use crate::RequestConfig;

use axum::body;
use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::Response;

pub async fn full(State(req_cfg): State<RequestConfig>, request: Request) -> Response {
    println!("----------------------------------------");
    println!("URI: {}", request.uri());
    println!("METHOD: {}", request.method());
    if !req_cfg.noout {
        println!("========== HEADER ==========");
        for (key, val) in request.headers().into_iter() {
            println!("{}:{:?}", key, val);
        }
        if request.method() == Method::POST {
            println!("=========== BODY ===========");
            let body_result = body::to_bytes(request.into_body(), req_cfg.bytes).await;
            match body_result {
                Ok(body) => println!("{:?}", body),
                Err(e) => println!("{}", e),
            }
        }
    }
    println!("============================");
    let mut builder = Response::builder().status(StatusCode::OK);
    for (key, val) in req_cfg.response_headers {
        builder = builder.header(&key[..], &val[..]);
    }
    match builder.body(Body::from(req_cfg.response_body)) {
        Ok(result) => result,
        Err(e) => {
            println!("Cannot construct response: {}", e);
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(""))
                .unwrap()
        }
    }
}

pub async fn empty() {}
