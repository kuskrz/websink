mod handles;
mod router;

use crate::router::init_router;

use std::{fs, net::SocketAddr, path::PathBuf};

use axum_server::tls_rustls::RustlsConfig;
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

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
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
    /*  let socket = String::from("0.0.0.0:") + &args.port.to_string();
    println!("Listening on {}", socket);
    // unwrap - no sense to start without socket and server
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app).await.unwrap(); */

    // https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs
    let ports = Ports {
        http: 7878,
        https: 3000,
    };

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
