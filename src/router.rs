use crate::handles::{empty, fullg, fullp};

use axum::routing::get;
use axum::{routing::post, Router};

#[derive(Clone)]
pub struct RequestConfig {
    pub bytes: usize,
    pub sink: bool,
    pub noout: bool,
    pub response: Option<String>,
}

pub fn init_router(args: RequestConfig) -> Router {
    if args.sink {
        print!("Sink mode! ");
        return Router::new().route("/", post(empty)).route("/", get(empty));
    }
    Router::new()
        .route("/", post(fullp))
        .route("/", get(fullg))
        .with_state(args)
}
