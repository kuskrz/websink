use axum::body;
use axum::{
    body::Bytes,
    extract::{Path, Request},
    response::IntoResponse,
    routing::post,
    Router,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// port number
    #[arg(short = 'p', long = "port", default_value_t = 7171)]
    port: i32,

    /// do not produce stdout
    #[arg(short = 'n', long = "noout", default_value_t = true)]
    noout: bool,

    /// response file path
    #[arg(short, long)]
    response: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("nout: {}", args.noout);
    match args.response {
        Some(v) => println!("response: {}", v),
        None => println!("response: empty"),
    }
    println!("port: {}", args.port);

    let app = init_router();
    let socket = String::from("0.0.0.0:") + &String::from(args.port.to_string());
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_router() -> Router {
    Router::new().route("/", post(full_request))
}

async fn full_request(request: Request) {
    println!("========== HEADER ==========");
    for (key, val) in request.headers().into_iter() {
        println!("{}:{:?}", key, val);
    }

    println!("=========== BODY ===========");
    //let body = request.into_body();
    let body = body::to_bytes(request.into_body(), 10000).await.unwrap();
    println!("{:?}", body);
}
