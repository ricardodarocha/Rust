# Rust

Um repositório para estudar Rust do começo  

---

Este material começou com meu interesse pessoal em aprender Rust.    
Aos poucos está se tranformando também num guia para que outras pessoas que tem interesse aprendam Rust.  
Me ajude a entender o seu perfil e responda a esta pesquisa.  
https://forms.gle/g5Y6V3g5ag75qcPg8  

# Prefácio

Se você não tem conhecimento de nenhuma linguagem de programação, é recomendado aprender os fundamentos primeiro. Aprenda **Python** ou **C** para compreender como as linguagens de programação funcionam.  
**Rust** é uma linguagem excelente, mas por possuir vários recursos avançados, não é recomedável que você comece por ela. É recomendável que você aprenda **Rust** como segunda linguagem até se tornar experiente.  

# Intro

Você pode facilmente aprender **Rust** na internet, lendo o Livro de Rust, acessando a Documentação ou fazendo cursos pela internet.  
Para exercitar você precisa configurar um ambiente. 
[Diversas formas de configurar um ambiente Rust](https://github.com/ricardodarocha/Rust/edit/main/Ambiente.md)

# Hello World

A forma mais comum de aprender uma linguagem é começar pelo exemplo _Olá mundo_.  

```Rust
//const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    println!("Olá, mundo!");
}
```
Há algumas variações deste programa que eu explico aqui, com o objetivo de incrementalmente ir aprendendo novos recursos. Vou utilizar comentários no código para explicar. Veja o tutorial sobre [Como comentar códigos Rust](https://github.com/ricardodarocha/Rust/edit/main/Documentando.md).


```Rust

/// Disponibilizando diferentes versões para o seu usuário
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    /// neste exemplo, além de aprender a exibir a versão você também aprendeu como formatar valores na mensagem
    println!("Olá, mundo! \nThis is version {}", VERSION);
}
```

# Aprendendo de A a Z

Nesta seção eu organizo os conteúdos por ordem alfabética. Se preferir uma sequência de estudos acompanhe o tutorial a seguir.

# Continuando o tutorial

A sequência de estudo que eu tenho aprimorado para você que está começando do zero é a seguinte. E eu vou tentar explicar a razão disso:

## Ler um arquivo de configurações .env

É bem provável que algumas configurações você possa controlar pelo próprio cargo.toml que eu expliquei no exemplo hello world, como a versão, o autor e outras informações do binário.
Outras informações é interessante deixar em um arquivo .env, que você pode ler facilmente com o tutorial a seguir

```Rust
todo!()
```

## Salvar um arquivo localmente

Ao interagir com o seu programa o usuário o alimenta com dados. Muitas vezes é conveniente armazenar estes dados para serem recuperados no futuro, mesmo quando o programa é fechado e após abrí-lo novamente, o usuário pode precisar reutilizar estes dados ou compartilhá-los com outros dispositivos por exemplo. Neste exemplo eu mostro como salvar dados localmente ou em rede.

## Ler arquivos 

Da mesma forma é necessário ler os dados gravados, ou então em alguns casos você vai querer ler arquivos que foram gerados por outros dispositivos e importá-los no seu sistema.

Há duas formas principais de ler estes arquivos, que eu divido em 

  [Ler um arquivo simples]() A maneir mais prática mas nem sempre resolve.  
  [Ler um arquivo grande no formato de stream]() Esta maneira poderoso permite gerenciar o uso de memória e ler arquivos gigantes. 

## Criar aplicativos de Console ou de Linha de Comando CLI

Eu possuo um tutorial mais completo sobre CLI no repo [cli with rust]  
No entanto aqui eu quero dar uma breve introdução:

## Criando interfaces mais amigáveis e melhores

Se você quer começar logo veja meu tutorial de [Interfaces Amigáveis com Rust]()

Aqui nós experimentamos algums recursos básicos de interface. Mesmo utilizando CLI é possível criar boas interfaces. Em seguida avanço um pouco mais criando interfaces "bonitas" com TUI que são interfaces visuais usando ASCII (exclusivo para terminais e aplicações de linhas de comando).

Se pretende criar interfaces mais avançadas como UI do Sistema operacional como Janelas do Windows, componentes Nativos eu também tenho este estudo. [Interfaces Nativas com Rust]()

Também estou desenvolvendo este material sobre o Estado da Arte das Interfaces com Rust e componentes Web com [Interfaces com Rust - Estado da Arte]()

Se você tem interesse por games dê uma olhada nestes [Experimentos com Rust - Games]()

## Lendo arquivos CSV com Rust

## Lendo JSON com Rust

## Trabalhando com Bancos de dados

Nesta seção eu dedico uma parte especial ao SQLite.
Em seguida eu recomendo você experimentar um banco de dados Profissional que suporta grande volume de dados como Postgresql
[Trabalhando com Banco de Dados]

Neste diretório eu reúno um estudo completo de SQL com Rust, incluindo discutindo alguns fundamentos de SQL [SQL completo com Rust]
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

# Arquiteturas com RUST

Nesta seção eu reúno meus estudos mais recentes sobre Arquiteturas com Rust, inclusive comparando com outras linguagens e discutindo as vantanges e desvantanges de cada um. Eu convido fortemente a comunidade a participar desta discussão aqui [link]

# Links

**github notebook**(Best Rust curated)[https://github.com/brson/rust-anthology/blob/master/master-list.md]
**Video** (Error Handling)[https://www.youtube.com/watch?v=mhw-x5Q_-z0&t=195s]

https://lib.rs/crates/cursive
