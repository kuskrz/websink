use crate::handles::{empty, fullg, fullp};
use crate::RequestConfig;

use axum::routing::get;
use axum::{routing::post, Router};

pub fn init_router(args: RequestConfig) -> Router {
    if args.sink {
        print!("Sink mode! ");
        return Router::new().route("/", post(empty)).route("/", get(empty));
    }
    Router::new()
        .route("/", post(fullp))
        .route("/", get(fullg))
        .route("/*path", post(fullp))
        .route("/*path", get(fullg))
        .with_state(args)
}
