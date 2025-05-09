use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

// This function is called for every HTTP request:
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("→ {} {}", req.method(), req.uri().path());

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::new(Body::from("Hello, async world!")),
        _ => {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            not_found
        }
    };
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Bind address
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    println!("Async server running on http://{}", addr);

    // Wrap the handler in a make_service_fn
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    // Start the server
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
