use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

use std::env;

#[tokio::main]
async fn main() {
    let key = "RUST_DOCKER_NAME";

    // print Hello World if ENV name not present
    match env::var(key) {
        Ok(val) => println!("Hello, {}", val),
        _ => println!("Hello World"),
    }
    ////////////////////
    let health_route = warp::path!("health").and_then(health_handler);

    let routes = health_route.with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}
