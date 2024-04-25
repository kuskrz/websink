use crate::handles::{empty, full};
use crate::RequestConfig;

use axum::routing::get;
use axum::{routing::post, Router};
use colored::Colorize;

pub fn init_router(args: RequestConfig) -> Router {
    if args.sink {
        print!("{} ", "Sink mode!".yellow());
        return Router::new()
            .route("/", post(empty))
            .route("/", get(empty))
            .route("/*path", post(empty))
            .route("/*path", get(empty));
    }
    Router::new()
        .route("/", post(full))
        .route("/", get(full))
        .route("/*path", post(full))
        .route("/*path", get(full))
        .with_state(args)
}
