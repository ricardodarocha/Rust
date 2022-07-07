# RUST API

## Intro

🐤 Este é um guia para **criação de APIS com RUST**

🐘 Acesse o guia mais recente para criar [**APIS com RUST + Postgres**](https://github.com/ricardodarocha/sqlxpg)

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

## Tutorial Completo

Neste exemplo completo nós iremos criar uma API com RUST que poderá ser acessada em uma porta e retornar um JSON
Possui também recursos de navegação, onde em cada rota acessada com método GET será retornado um JSON específico
Nos próximos tutoriais serão criadas outras funcionalidades como uma rota com método POST PUT e DELETE

Abrir a linha de comandos em uma pasta de projetos e dentro dela rodar o comando

```shell
cargo new api_name --bin
code .
```

# Veja mais

Trabalhando com [JSON]()
Veja [como criar módulos](https://github.com/ricardodarocha/Rust/edit/main/README.md#Modules)
