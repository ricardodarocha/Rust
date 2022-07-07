# RUST API

## Intro

🐤 Este é um exemplo para **criar APIS com o RUST**

🐘 Acesse o guia mais recente para criar [**APIS com RUST + Postgres**](https://github.com/ricardodarocha/sqlxpg)

Neste exemplo eu mostro como criar uma API do zero
``` 
💬 Há várias maneiras de criar APIS com Rust, inclusive há diversas libs entre elas Actix, Tokio, Axum,  
   Algumas possuem suporte a assíncrono, threads e outros recursos. Neste exemplo eu criei   
   a API da maneira mais simples possível, sem instalar nenhuma lib externa e sem complicações  
```

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

Neste exemplo eu recomendo que você comece com dois arquivos, ou seja
a regra de negócios principal fica no main.rs, onde você expõe o servidor e suas configurações básicas, como a porta.
As rotas você deve deixar no arquivo routes.rs.
Eu recomendo criar as rotas em um arquivo separado porque um projeto de API pode se tornar grande com o tempo, e mesmo para projetos pequenos, você criar uma melhor separação dos arquivos.

## Tutorial Completo

Neste exemplo de API iremos retornar um Json  

Esta api possui rotas que podem ser acessadas com método GET  
Para cada rota, será retornado um JSON específico e uma mensagem de sucesso 200 OK  
Nos próximos tutoriais serão criadas outras funcionalidades como uma rota com método POST PUT e DELETE, e retornos personalizados  

🦀
Abrir a linha de comandos em uma pasta de projetos e dentro dela rodar o comando

```shell
cargo new api_name --bin
code .
```

# Veja mais

Trabalhando com [JSON]()
Veja [como criar módulos](https://github.com/ricardodarocha/Rust/edit/main/README.md#Modules)
