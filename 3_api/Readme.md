# RUST API

## Intro

Este é um guia para criação de APIS com RUST

Neste exemplo eu mostro como criar uma API do zero.

Comece com o arquivo `main.rs`

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

Neste exemplo eu recomendo você começar com dois arquivos, ou seja
a regra de negócios principal fica no main.rs, onde você expõe o servidor e suas configurações básicas, como a porta.
As rotas você deve deixar no arquivo routes.rs.
Eu recomendo criar as rotas em um arquivo separado porque um projeto de API pode se tornar grande com o tempo, e mesmo para projetos pequenos, você criar uma melhora separação das responsabilidades.

Veja [como criar módulos](https://github.com/ricardodarocha/Rust/edit/main/README.md#Modules)
