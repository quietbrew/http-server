mod handlers;
mod router;

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::route)) });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
