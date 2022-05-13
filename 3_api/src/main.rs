//Run > cargo run 8085

use actix_web::{web, App, HttpServer};
use std::env;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let port = &args[1];
    let port: u16 = match port.parse() {
        Ok(number) => {
            number
        },
        Err(_) => {
            8000
        },
    };

    println!("Running on port ... http://localhost:{port}");
    
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}   