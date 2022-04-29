# Rust

A repository to study Rust from scratch

# Intro

Point of view analisys about the language, the benefits and what you can expect about.

# Hello World

The first example you can have
In hello world 2 I improve the example to shows the version of compiled package.  You can have a look at [Cargo.lock] file

```Rust
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    println!("Hello, world! \nThis is version {}", VERSION);
}
```

# Api 

This is a setup to create API

This amazing example shows how easy is to create an Api from scratch.

Have a tasting of how the api works

This is `main.rs`

```Rust
HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
```

I create endpoints in file routes.rs
---

# Sumary by Subject

## Variables

## Types

## Modules

```Rust
-main.rs-
mod routes; //A file with name routes.rs

---
-routes.rs-
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
```

