# Rust

A repository to study Rust from scratch

# Intro

Point of view analisys about the language, the benefits and what you can expect of.

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

see [row to create modules](https://github.com/ricardodarocha/Rust/edit/main/README.md#Modules)

# Log

In this example I explore the funcionalities of crate LOG, and how to show into console the status of server.
It's a continuation of example 3_api

---

# Sumary by Subject

The examples by subject are in \General Folder

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

## Loop expressions

```Rust
for x in 1..11{ // 11 is not inclusive
      if x%2==0 {println!("{x} is even")}
      if x==5 {
         continue;
      println("{x}")
      
      if x==5 {
         break x //If is a function, returns x
         }
      }
```

Loops in details [](https://github.com/ricardodarocha/Rust/blob/main/general/loopExpressions)

## Error Handling

```Rust
fn exit(x: i32) {
    if x == 0 {
        panic!("we got a 0");
    }
    println!("things are fine")
}
```

Check [erro handling](https://github.com/ricardodarocha/Rust/blob/main/general/errorhandling.md) studies

## Writing tests and documentations

See this Topic [here](https://github.com/ricardodarocha/Rust/blob/main/general/tests_and_docs)

## env

Environments can be controlled using **Argument Parameters** and **Environment Variables**

### Reading Args

```Rust
/// In this example we intercept first argunt sent by the user when calling the executable
/// ```shell
/// cargo run --parameter
/// ```
use std::env;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let port = &args[1];
```

### Env Variables

```Rust
/// Now we just read the values stored in .env file
/// ```ini
/// user_name=admin
/// ```
use dotenv;
dotenv().expect(".env file not found");
println!("KEY", env::var("user_name").unwrap());
env::set_var("password", "unbush84likely8Fdetail42");
```

###  TUI

This is how we can create amazing TUI Text User Interfaces with Rust.

[blue TUI](https://lib.rs/crates/cursive)
[term-table](https://lib.rs/crates/term-table)

A lot of other [TUI libraries](https://lib.rs/command-line-interface)


# Links

**github notebook**(Best Rust curated)[https://github.com/brson/rust-anthology/blob/master/master-list.md]
**Video** (Error Handling)[https://www.youtube.com/watch?v=mhw-x5Q_-z0&t=195s]

https://lib.rs/crates/cursive
