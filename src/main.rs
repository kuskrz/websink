mod command;
mod handles;
mod response;
mod router;

use crate::response::ResponseTOML;
use crate::router::init_router;

use std::{net::SocketAddr, path::PathBuf, process};

use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use colored::Colorize;

#[derive(Clone)]
struct RequestConfig {
    bytes: usize,
    sink: bool,
    noout: bool,
    response_toml: ResponseTOML,
    delay: u16,
}

#[tokio::main]
async fn main() {
    let args = command::Args::parse();
    let mut response_toml = ResponseTOML::new_empty();
    if !args.sink {
        if let Some(file_name) = args.response {
            response_toml = ResponseTOML::parse_response(&file_name[..]);
        }
    }

    let request_config = RequestConfig {
        bytes: args.bytes,
        sink: args.sink,
        noout: args.noout,
        response_toml,
        delay: args.delay,
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
            println!("{}: key file does not exist", "ERROR".red());
            process::exit(2);
        }
        let cert = args.cert.expect("cet not set");
        let cert_file = PathBuf::from(cert);
        if !cert_file.exists() {
            println!("{}: cert file does not exist", "ERROR".red());
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
