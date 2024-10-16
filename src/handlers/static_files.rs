use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn handle(path: &str) -> Result<Response<Body>, Infallible> {
    let path = path.trim_start_matches("/static");
    let path = format!("./static{}", path); // Adjust this to your static files directory
    match File::open(path).await {
        Ok(file) => {
            let stream = FramedRead::new(file, BytesCodec::new());
            let body = Body::wrap_stream(stream);
            Ok(Response::new(body))
        }
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap()),
    }
}
