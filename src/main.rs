mod handles;
mod router;

use crate::router::init_router;

use std::{fs, net::SocketAddr, path::PathBuf, process};

use axum_server::tls_rustls::RustlsConfig;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// port number
    #[arg(short = 'p', long = "port", default_value_t = 2024)]
    port: u16,

    /// key file
    #[arg(short = 'k', long = "key")]
    key: Option<String>,

    /// cert file
    #[arg(short = 'c', long = "cert")]
    cert: Option<String>,

    /// sink mode
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
    println!("\u{1f980} ");
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

    if args.key.is_none() || args.cert.is_none() {
        let socket = String::from("0.0.0.0:") + &args.port.to_string();
        println!("Listening on {}", socket);
        // unwrap - no sense to start without socket and server
        let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        // https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs
        let key = args.key.expect("key not set");
        let key_file = PathBuf::from(key);
        if !key_file.exists() {
            println!("key file does not exist");
            process::exit(2);
        }
        let cert = args.cert.expect("cet not set");
        let cert_file = PathBuf::from(cert);
        if !cert_file.exists() {
            println!("cert file does not exist");
            process::exit(2);
        }

        let config = RustlsConfig::from_pem_file(cert_file, key_file)
            .await
            .unwrap();

        let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
