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

# Loop expressions:

Rust supports four loop expressions:
- A **loop** expression denotes an infinite loop.
- A **while** expression loops until a predicate is false.
- A **while let** expression tests a pattern.
- A **for** expression extracts values from an iterator, looping until the iterator is empty.
All four types of loop support break expressions, continue expressions, and labels. Only loop supports evaluation to non-trivial values.

### Loop
``` Rust
loop { println!("I live."); }
```

### while
``` Rust
let mut i = 0;

while i < 10 {
    println!("hello");
    i = i + 1;
}
```

### while let
``` Rust
let mut x = vec![1, 2, 3];

while let Some(y) = x.pop() {
    println!("y = {}", y);
}

```

Multiple patterns may be specified with the | operator. This has the same semantics as with | in match expressions:

``` Rust
let mut vals = vec![2, 3, 1, 2, 2];
while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
    // Prints 2, 2, then 1
    println!("{}", v);
}
```

### For
A for expression is a syntactic construct for looping over elements provided by an implementation of std::iter::IntoIterator. If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the for loop. If the iterator is empty, the for expression completes.

An example of a for loop over the contents of an array:

``` Rust
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
```

An example of a for loop over a series of integers:

``` Rust
let mut sum = 0;
for n in 1..11 {
    sum += n;
}
assert_eq!(sum, 55);
```
