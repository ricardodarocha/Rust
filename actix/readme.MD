# RUST Api Actix level 2

## Intro

🐶 Este é um exemplo de um nível básico, mas exige um pouco mais de experiência

Neste exemplo eu mostro algumas features adicionais como uuid, chrono

```shell
cargo add api_name & cd api_name
cargo add actix-web
cargo add serde --features derive
cargo add serde_json
cargo add chrono --features serde
cargo add uuid --features v4
```

Irá produzir o seguinte **cargo.toml**  

```rust
[dependencies]
actix-web = "4.3.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4.26", features = ["serde"] }
uuid = { version = "1.4.1", features = ["v4"] }
```

Para produzir uma api básica cole o código no **main.rs**  

```rust
///para mais informações consulte https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/

use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Nf{
   id: u32,
   author: String,
}

// Implement Responder Trait for Nf
impl Responder for Nf {
   type Body = BoxBody;

   fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
       let res_body = serde_json::to_string(&self).unwrap();

       // Create HttpResponse and set Content Type
       HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
   }
}

#[derive(Debug, Serialize)]
struct ErrNoId {
   id: u32,
   err: String,
}

// Implement ResponseError for ErrNoId
impl ResponseError for ErrNoId {
   fn status_code(&self) -> StatusCode {
       StatusCode::NOT_FOUND
   }

   fn error_response(&self) -> HttpResponse<BoxBody> {
       let body = serde_json::to_string(&self).unwrap();
       let res = HttpResponse::new(self.status_code());
       res.set_body(BoxBody::new(body))
   }
}

// Implement Display for ErrNoId
impl Display for ErrNoId {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self)
   }
}

struct AppState {
   tickets: Mutex<Vec<Nf>>,
}

// Create a ticket
#[post("/tickets")]
async fn post_ticket(req: web::Json<Nf>, data: web::Data<AppState>) -> impl Responder {
   let new_ticket = Nf {
       id: req.id,
       author: String::from(&req.author),
   };

   let mut tickets = data.tickets.lock().unwrap();

   let response = serde_json::to_string(&new_ticket).unwrap();

   tickets.push(new_ticket);
   HttpResponse::Created()
       .content_type(ContentType::json())
       .body(response)
}

// Get all tickets
#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
   let tickets = data.tickets.lock().unwrap();

   let response = serde_json::to_string(&(*tickets)).unwrap();

   HttpResponse::Ok()
       .content_type(ContentType::json())
       .body(response)
}

// Get a ticket with the corresponding id
#[get("/tickets/{id}")]
async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Nf, ErrNoId> {
   let ticket_id: u32 = *id;
   let tickets = data.tickets.lock().unwrap();

   let ticket: Vec<_> = tickets.iter()
                               .filter(|x| x.id == ticket_id)
                               .collect();

   if !ticket.is_empty() {
       Ok(Nf {
           id: ticket[0].id,
           author: String::from(&ticket[0].author)
       })
   } else {
       let response = ErrNoId {
           id: ticket_id,
           err: String::from("ticket not found")
       };
       Err(response)
   }
}

// Update the ticket with the corresponding id
#[put("/tickets/{id}")]
async fn update_ticket(id: web::Path<u32>, req: web::Json<Nf>, data: web::Data<AppState>) -> Result<HttpResponse, ErrNoId> {
   let ticket_id: u32 = *id;

   let new_ticket = Nf {
       id: req.id,
       author: String::from(&req.author),
   };

   let mut tickets = data.tickets.lock().unwrap();

   let id_index = tickets.iter()
                         .position(|x| x.id == ticket_id);

   match id_index {
       Some(id) => {
           let response = serde_json::to_string(&new_ticket).unwrap();
           tickets[id] = new_ticket;
           Ok(HttpResponse::Ok()
               .content_type(ContentType::json())
               .body(response)
           )
       },
       None => {
           let response = ErrNoId {
               id: ticket_id,
               err: String::from("ticket not found")
           };
           Err(response)
       }
   }
}

// Delete the ticket with the corresponding id
#[delete("/tickets/{id}")]
async fn delete_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Nf, ErrNoId> {
   let ticket_id: u32 = *id;
   let mut tickets = data.tickets.lock().unwrap();

   let id_index = tickets.iter()
                         .position(|x| x.id == ticket_id);

   match id_index {
       Some(id) => {
           let deleted_ticket = tickets.remove(id);
           Ok(deleted_ticket)
       },
       None => {
           let response = ErrNoId {
               id: ticket_id,
               err: String::from("ticket not found")
           };
           Err(response)
       }
   }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let app_state = web::Data::new(AppState {
                        tickets: Mutex::new(vec![
                            Nf {
                                id: 1,
                                author: String::from("Jane Doe")
                            },
                            Nf {
                                id: 2,
                                author: String::from("Patrick Star")
                                }
                        ])
                    });

   HttpServer::new(move || {
       App::new()
           .app_data(app_state.clone())
           .service(post_ticket)
           .service(get_ticket)
           .service(get_tickets)
           .service(update_ticket)
           .service(delete_ticket)
   })
   .bind(("127.0.0.1", 8000))?
   .run()
   .await
}
```
