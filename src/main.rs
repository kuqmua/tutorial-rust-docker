// use std::env;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::thread;

// fn handle_read(mut stream: &TcpStream) {
//     let mut buf = [0u8; 4096];
//     match stream.read(&mut buf) {
//         Ok(_) => {
//             let req_str = String::from_utf8_lossy(&buf);
//             println!("{}", req_str);
//         }
//         Err(e) => println!("Unable to read stream: {}", e),
//     }
// }

// fn handle_write(mut stream: TcpStream) {
//     let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
//     match stream.write(response) {
//         Ok(_) => println!("Response sent"),
//         Err(e) => println!("Failed sending response: {}", e),
//     }
// }

// fn handle_client(stream: TcpStream) {
//     handle_read(&stream);
//     handle_write(stream);
// }

// fn main() {
//     let key = "RUST_DOCKER_NAME";
//     match env::var(key) {
//         Ok(val) => {
//             println!("there is an environment variable {}: {}!", key, val);
//         }
//         _ => println!("there is no environment variable..."),
//     }
//     main_tokio();
//     let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
//     println!("Listening for connections on port {}", 8000);
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 thread::spawn(|| handle_client(stream));
//             }
//             Err(e) => {
//                 println!("Unable to connect: {}", e);
//             }
//         }
//     }
// }
//////////////////////////////////////////////////
// use warp::{Filter, Rejection, Reply};

// type Result<T> = std::result::Result<T, Rejection>;

// #[tokio::main]
// async fn main_tokio() {
//     println!("Hello world1");
//     let health_route = warp::path!("health").and_then(health_handler);
//     let routes = health_route.with(warp::cors().allow_any_origin());
//     println!("Started server at localhost:8000");
//     warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
//     println!("Hello world3");
// }

// async fn health_handler() -> Result<impl Reply> {
//     Ok("OK")
// }
//////////////////////////////////////////////////
use std::{thread, time};

#[tokio::main]
async fn main() {
    let two_seconds = time::Duration::from_millis(2000);
    loop {
        println!("test");
        thread::sleep(two_seconds);
    }
}
