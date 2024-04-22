mod handles;
mod response;
mod router;

use crate::response::parse_response;
use crate::router::init_router;

use std::{net::SocketAddr, path::PathBuf, process};

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
    response_headers: Vec<(String, String)>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut responseb = "".to_owned();
    let mut responseh = Vec::new();

    if !args.sink {
        if let Some(file_name) = args.response {
            let (response_body_toml, response_headers_toml) = parse_response(&file_name[..]);
            responseb = response_body_toml.unwrap_or_default();
            if let Some(h) = response_headers_toml {
                responseh = h
                    .into_iter()
                    .map(|x| (x.0, String::from(x.1.as_str().unwrap_or_default())))
                    .collect();
            }
        }
    }

    let request_config = RequestConfig {
        bytes: args.bytes,
        sink: args.sink,
        noout: args.noout,
        response_body: responseb,
        response_headers: responseh,
    };

    let app = init_router(request_config);

    if args.key.is_none() || args.cert.is_none() {
        let socket = String::from("0.0.0.0:") + &args.port.to_string();
        println!("Listening on http://{} \u{1f980}", socket);
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
        println!("Listening on https://0.0.0.0:{} \u{1f980}", args.port);
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
