# RUST API

## Intro

🐤 Este é um exemplo para **criar APIS com o RUST**

🐘 Acesse também esta dica recente para criar [**APIS com RUST + Postgres**](https://github.com/ricardodarocha/sqlxpg)

Neste exemplo eu mostro como criar uma API do zero
``` 
💬 Há várias maneiras de criar APIS com Rust, inclusive há diversas libs entre elas Actix, Tokio, Axum,  
   Algumas possuem suporte a assíncrono, threads e outros recursos. Neste exemplo eu criei   
   a API da maneira mais simples possível, com Actix
```

Comece com o arquivo `main.rs`

```Rust
HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .route("/hey", web::get().to(routes::hello_world))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
```

em seguida registre suas rotas 

```Rust
#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
```

e rode o comando **cargo run** no shell

Neste exemplo eu recomendo que você comece com dois arquivos, ou seja
a regra de negócios principal fica no main.rs, onde você expõe o servidor e suas configurações básicas, como a porta.
As rotas você deve deixar no arquivo routes.rs.
Eu recomendo criar as rotas em um arquivo separado porque um projeto de API pode se tornar grande com o tempo, e mesmo para projetos pequenos, você criar uma melhor separação dos arquivos.

## Tutorial Completo

Neste tutorial mais detalhado eu mostro como criar o projeto utilizando o CMD e como compilar os binários  
🦀  
Abrir a linha de comandos em uma pasta de projetos e dentro dela rodar o comando

```shell
cargo new api_name --bin
code .
```

🚧 _em manutenção_

# Veja mais

Trabalhando com [JSON]()
Veja [como criar módulos](https://github.com/ricardodarocha/Rust/edit/main/README.md#Modules)
