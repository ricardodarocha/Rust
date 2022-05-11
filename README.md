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



# Links

**github notebook**(Best Rust curated)[https://github.com/brson/rust-anthology/blob/master/master-list.md]
**Video** (Error Handling)[https://www.youtube.com/watch?v=mhw-x5Q_-z0&t=195s]
