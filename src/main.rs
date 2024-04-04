mod handles;
mod router;

use crate::router::init_router;

use std::fs;

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
    response_body: String,
}

#[tokio::main]
async fn main() {
    print!("\u{1f980} ");
    let args = Args::parse();
    let mut response = "".to_owned();

    if !args.sink {
        if let Some(file_name) = args.response {
            if let Ok(fc) = fs::read_to_string(file_name) {
                response = fc;
            }
        }
    }

    let request_config = RequestConfig {
        bytes: args.bytes,
        sink: args.sink,
        noout: args.noout,
        response_body: response,
    };

    let app = init_router(request_config);
    let socket = String::from("0.0.0.0:") + &args.port.to_string();
    println!("Listening on {}", socket);
    // unwrap - no sense to start without socket and server
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
