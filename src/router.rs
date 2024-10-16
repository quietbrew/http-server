use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use crate::handlers::{home, about, static_files};

pub async fn route(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => home::handle().await,
        (&Method::GET, "/about") => about::handle().await,
        (&Method::GET, path) if path.starts_with("/static/") => static_files::handle(path).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap())
    }
}