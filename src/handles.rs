use crate::RequestConfig;

use axum::body::{to_bytes, Body};
use axum::extract::Request;
use axum::extract::State;
use axum::http::Method;
use axum::http::StatusCode;
use axum::response::Response;
use colored::Colorize;
use tokio::time::{sleep, Duration};

pub async fn full(State(req_cfg): State<RequestConfig>, request: Request) -> Response {
    let uristr = request.uri().to_string();
    let methstr = request.method().to_string();

    println!("{}", "REQUEST BEGIN".bold());
    println!(" {}: {}", "URI".green(), uristr);
    println!(" {}: {}", "METHOD".green(), methstr);
    if !req_cfg.noout {
        println!(" {}:", "HEADER".green());
        for (key, val) in request.headers().into_iter() {
            println!("  {}:{:?}", key, val);
        }
        if request.method() == Method::POST || request.method() == Method::PUT {
            println!(" {}:", "BODY".green());
            let body_result = to_bytes(request.into_body(), req_cfg.bytes).await;
            match body_result {
                Ok(body) => println!("{}", String::from_utf8_lossy(body.as_ref())),
                Err(e) => println!("{}", e),
            }
        }
    }

    let mut builder = Response::builder().status(StatusCode::OK);

    if let Some(headers) = req_cfg.response_toml.get_headers(&uristr[..], &methstr[..]) {
        println!(" {}:", "RESPONSE HEADER".bright_green());
        for (key, val) in headers {
            println!("  {}:{:?}", key, val);
            builder = builder.header(&key[..], &val[..]);
        }
    }

    if let Some(body) = req_cfg.response_toml.get_body(&uristr[..], &methstr[..]) {
        println!(" {}:", "RESPONSE BODY".bright_green());
        println!("{}", body);
    }

    if req_cfg.delay > 0 && req_cfg.delay < 60000 {
        println!(" {}: {}ms", "DELAY BEGIN".yellow(), req_cfg.delay);
        sleep(Duration::from_millis(req_cfg.delay as u64)).await;
        println!(" {}", "DELAY END".yellow());
    }

    println!("{}\n", "REQUEST END".bold());

    let mut response_to_send = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(""))
        .unwrap();
    if let Some(body) = req_cfg.response_toml.get_body(&uristr[..], &methstr[..]) {
        match builder.body(Body::from(body)) {
            Ok(result) => response_to_send = result,
            Err(e) => {
                println!("{}: Cannot construct response: {}", "ERROR".red(), e);
            }
        }
    }
    response_to_send
}

pub async fn empty() {}
