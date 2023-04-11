use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = json!({
        "message": "Hello from the API!"
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap())
}

#[tokio::main]
async fn main() {
    // bind to 127.0.0.1:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // create boiler plate handler service for hello world request
    let make_service = make_service_fn(|_conn| async {
        // convert handler into a service
        Ok::<_, Infallible>(service_fn(handle_hello_world))
    });

    // create server
    let server = Server::bind(&addr).serve(make_service);

    println!("Server is running on http://127.0.0.1:3000");

    let graceful_halt = 
        server.with_graceful_shutdown(shutdown_signal());

    // run forever
    if let Err(e) = graceful_halt.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}