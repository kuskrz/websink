use std::fs;

use axum::body;
use axum::extract::State;
use axum::routing::get;
use axum::{extract::Request, routing::post, Router};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// port number
    #[arg(short = 'p', long = "port", default_value_t = 2024)]
    port: u32,

    /// do not produce stdout
    #[arg(
        short = 's',
        long = "sink",
        action(ArgAction::SetTrue),
        default_value_t = false
    )]
    sink: bool,

    /// do not produce stdout
    #[arg(
        short = 'n',
        long = "noout",
        action(ArgAction::SetTrue),
        default_value_t = false
    )]
    noout: bool,

    /// body maximum size in bytes
    #[arg(short = 'b', long = "bytes", default_value_t = 10240)]
    bytes: usize,

    /// response file path
    #[arg(short, long)]
    response: Option<String>,
}

#[derive(Clone)]
struct RequestConfig {
    bytes: usize,
    sink: bool,
    noout: bool,
    response: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    /* println!("nout: {}", args.noout);
    match &args.response {
        Some(v) => println!("response: {}", v),
        None => println!("response: empty"),
    }
    println!("port: {}", args.port); */

    let request_config = RequestConfig {
        bytes: args.bytes,
        sink: args.sink,
        noout: args.noout,
        response: args.response,
    };

    let app = init_router(request_config);
    let socket = String::from("0.0.0.0:") + &String::from(args.port.to_string());
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_router(args: RequestConfig) -> Router {
    if args.sink {
        println!("Sink mode");
        return Router::new().route("/", post(empty)).route("/", get(empty));
    }
    Router::new()
        .route("/", post(fullp))
        .route("/", get(fullg))
        .with_state(args)
}

async fn fullp(State(req_cfg): State<RequestConfig>, request: Request) -> String {
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

async fn fullg(State(req_cfg): State<RequestConfig>) -> String {
    let mut return_string = String::from("");
    if let Some(file_name) = req_cfg.response {
        if let Ok(file_content) = fs::read_to_string(file_name.to_owned()) {
            return_string = file_content;
        }
    }

    return_string
}

async fn empty() {}
