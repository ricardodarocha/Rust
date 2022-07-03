# Rust

Um repositório para estudar Rust do começo  
🚧 **Em construção**  

---

Este material começou com meu interesse pessoal em aprender Rust.    
Aos poucos, está se tornando um guia para outras pessoas que querem aprender Rust.  
Se você quer aprender Rust comigo responda à pesquisa e me ajude a entender o seu perfil.  
https://forms.gle/g5Y6V3g5ag75qcPg8  

# Prefácio

Se você não tem conhecimento de qualquer linguagem de programação, é recomendado aprender os fundamentos primeiro.   
Aprenda **Python** ou **C** para compreender como as linguagens de programação funcionam.  
**Rust** é uma linguagem excelente, mas possui vários recursos avançados, e não é recomedável que você comece por ela. É recomendável que você aprenda **Rust** como segunda linguagem até se tornar experiente.  

# Intro

Você pode facilmente aprender **Rust** na internet, lendo o [Livro de Rust](https://livro.rustbr.org/) , acessando a [Documentação](https://www.rust-lang.org/pt-BR/) ou fazendo cursos pela internet.  
Para exercitar você precisa configurar um ambiente. 
[Diversas formas de configurar um ambiente Rust](https://www.rust-lang.org/pt-BR/tools/install)

# Olá Mundo

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

# Correspondência

o padrão **match** é um dos recursos mais importantes da linguagem Rust

```Rust
let x: i32 = 1;

match x {
  1 => println!("um"),
  2 => println!("dois"),
  3 => println!("três"),
  - => println!("outro qualquer"),
}
```

https://github.com/ricardodarocha/Rust/new/main/general/correspondencia.rs

# Aprendendo de A a Z

🚧 Nesta seção eu organizo os conteúdos por ordem alfabética. Se preferir uma sequência de estudos acompanhe o tutorial a seguir.

# Continuando o tutorial

A sequência de estudo que eu tenho aprimorado para você que está começando do zero é a seguinte. E eu vou tentar explicar a razão disso:

## Ler um arquivo de configurações .env

É comum controlarmos algumas configurações pelo próprio cargo.toml. Como eu expliquei no exemplo hello world, a versão, o autor e outras informações do binário podem ser declaradas explicitamente no arquivo cargo.toml.
Mas é interessante deixar outras informações em um arquivo .env, que você pode ler facilmente com o tutorial a seguir

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

## Ler os parâmetros da aplicação com args()

Ao executar uma aplicação pela linha de comandos, você pode passar parâmetros para ela
```Shell
c:\App> olamundo.exe ricardo
```

Você pode facilmente acessar estes parâmetros através do comando `args()`

```Rust
use std::env;
fn main() {
let args: Vec<_> = env::args().collect();
let name = match args.get(1) {
  Some(value) =>  value,
  None => "Mundo"
    };
println!("Olá {}!", name)
}
```

## Salvar um arquivo localmente

Ao interagir com o seu programa o usuário o alimenta com dados. Muitas vezes é conveniente armazenar estes dados para serem recuperados no futuro, mesmo quando o programa é fechado e após abrí-lo novamente, o usuário pode precisar reutilizar estes dados ou compartilhá-los com outros dispositivos por exemplo. Neste exemplo eu mostro como salvar dados localmente ou em rede.

Vamos continuar nosso exemplo e prepará-lo para exportar a mensagem em um arquivo

```Rust
use std::env;
fn main() {
let args: Vec<_> = env::args().collect();
let name = match args.get(1) {
  Some(value) =>  value,
  None => "Mundo"
    };
/// Parabéns, você aprendeu a formatar strings
let mensagem = format!("Olá {destinatario}!", destinatario = name)

/// E agora vamos salvar nossa mensagem em um arquivo externo
    let mut file = File::create("mensagem.txt")?;
    file.write_all(men sagem)?;
}
```

## Ler arquivos 

Da mesma forma é necessário ler os dados gravados, ou então em alguns casos você vai querer ler arquivos que foram gerados por outros dispositivos e importá-los no seu sistema.

Há duas formas principais de ler estes arquivos, que eu divido em 

  [Ler um arquivo simples]() A maneir mais prática mas nem sempre resolve.  
  [Ler um arquivo grande no formato de stream]() Esta maneira poderoso permite gerenciar o uso de memória e ler arquivos gigantes. 
  
Veja este exemplo básico

```Rust
    let mut file = File::open("mensagem.txt")?;
    let mut conteudo = String::new();
    file.read_to_string(&mut conteudo)?;
    assert_eq!(conteudo, "Hello, world!");
```

## Refatorando em métodos

Em qualquer linguagem de programação é uma boa prática manter o código limpo, e nunca criar funções muito grandes que tenham várias responsabilidades. Isto pode tornar o código confuso. Por isso nós vamos começar a refatorar o nosso código para quebrá-lo em vários métodos.

```Rust
use std::env;

fn salvar_no_arquivo(mensagem: String) {
  let mut file = File::create("mensagem.txt")?;
  file.write_all(mensagem)?;
 }

fn main() {

let args: Vec<_> = env::args().collect();
let name = match args.get(1) {
  Some(value) =>  value,
  None => "Mundo"
    };
let mensagem = format!("Olá {destinatario}!", destinatario = name)

salvar_no_arquivo(mensagem);
}
```

## Utilizando Linguagem Ubíqua

Em primeiro lugar, lendo o código acima, vemos que não é adequado manter blocos de código que estejam em níveis diferentes dentro da hierarquia de procedimentos, isto é, o código precisa fazer sentido como um todo par aquem lê. Isto nos faz um convite relacionado ao idioma, e de agora em diante vamos fazer um esforço para escrever todo o código em português. Por isso vamos encapsular todo o comportamento de setup do programa no método chamado `carregar_parametros`, isto nos permitirá no futuro utilizar um padrão de projetos (_design pattern_) muito interessante chamado **Builder**, que utiliza o conceito de _Fluent Api_. Mas não por enquanto.

```Rust
use std::env;

fn pegar_nome_usuario() -> String {
  let argumentos: Vec<_> = env::args().collect();
  let usuario match argumentos.get(1) {
    Some(valor) =>  valor,
    None => "Mundo"
    };
   return usuario;
}

fn formatar_mensagem(usuario) -> String {
  format!("Olá {destinatario}!", destinatario = usuario)
}

fn salvar_no_arquivo(mensagem: String) {
  let mut file = File::create("mensagem.txt")?;
  file.write_all(mensagem)?;
 }

fn main() {
let usuario = pegar_nome_usuario();
let mensagem = formatar_mensagem(usuario);
salvar_no_arquivo(mensagem);
}
```
A linguagem ubíqua é um conceito do DDD que prega ao desenvolvedor utilizar aspectos da língua falada ao escrever seu código, isto é, deve-se utilizar um formato de narrativa que se aproxime da língua dos usuários finais, utilizando inclusive as mesmas palavras que ele utiliza para descrever aquela rotina. Veja que ao utilizar esta técnica o código fica mais fluido, e mais simples de ler, um dos preceitos do código limpo.

## Outras lições da linguagem com este exemplo básico.

Em Rust você não é obrigado a utilizar **return**
Ao deixar o valor sem ponto e vírgula na última linha de uma função ele será retornando automaticamente

```Rust
fn pegar_nome_usuario() -> String {
  let args: Vec<_> = env::args().collect();
  let name match args.get(1) {
    Some(value) =>  value,
    None => "Mundo"
    };
   name
}
```

Isto dispensa o uso da variável name

```Rust
fn pegar_nome_usuario() -> String {
  let args: Vec<_> = env::args().collect();
  match args.get(1) {
    Some(value) =>  value,
    None => "Mundo"
    }
}
```

## Criar aplicativos de Console ou de Linha de Comando CLI

Eu estou desenvolvendo um tutorial mais completo sobre CLI no repo [cli with rust](?)🚧   
No entanto aqui eu vou dar uma breve introdução:

## Criando uma aplicação simples de linha de comando (CLI) 

Um aplicativo binário compilado pelo Rust pode ser facilmente integrado à Interface de Linha de comando de qualquer terminal, seja Linux, Windows ou Plataformas Embarcadas. Você pode chamar `curl aplicativo.exe` e ele será executado. Mas vamos ver o que podemos fazer para torná-lo mais interativo.

A primeira coisa é trabalhar com argumentos, ou parâmetros, como vimos no tutorial anterior.
Outra forma é coletar inputs do usuário, enquanto o programa está em execução.

```
fn pegar_nome_usuario() -> String {
  let args: Vec<_> = env::args().collect();
  match args.get(1) {
    Some(value) =>  value,
    None => {
      use std::io::{stdin};
      let mut usuario=String::new();
      print!("Digite seu nome: ");
      let _=stdout().flush();
      stdin().read_line(&mut usuario).wnrap_or("Mundo");
      usuario
      }
    }
}
```
Aprenda o estado da arte com [CLAP](https://docs.rs/clap/2.33.0/clap/)

## Criando interfaces mais amigáveis e melhores

🚧 Se você quer começar logo veja meu tutorial de [Interfaces Amigáveis com Rust]()

Aqui nós experimentamos algums recursos básicos de interface. Mesmo utilizando CLI é possível criar boas interfaces. Em seguida avanço um pouco mais criando interfaces "bonitas" com TUI que são interfaces visuais usando ASCII (exclusivo para terminais e aplicações de linhas de comando).

Se pretende criar interfaces mais avançadas como UI do Sistema operacional como Janelas do Windows, componentes Nativos eu também tenho este estudo. [Interfaces Nativas com Rust]()

Também estou desenvolvendo este material sobre o Estado da Arte das Interfaces com Rust e componentes Web com [Interfaces com Rust - Estado da Arte]()

Se você tem interesse por games dê uma olhada nestes [Experimentos com Rust - Games]()

# Coletâneas

Nesta série de coletâneas eu vou exibir exemplos mais práticos possíveis. Por conta disso, eu vou poupar entrar em detalhes.

## Envir um e-mail 

```Rust
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main()
{
    let email = Message::builder()
        .from("sender@...".parse().unwrap())
        .to("dest@...".parse().unwrap())
        .subject("Hello From Rust")
        .body(String::from("This is an automatic e-mail, please ignore!"))
        .unwrap();

    let creds = Credentials::new(
        "sender@...".to_string(),
        "password".to_string()
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
```

## Lendo arquivos CSV 

Consulte o tutorial completo da [https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html](documentação)
```Rust
let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
```

## Lendo JSON 

usando Serde
```Rust
fn main() {
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
}
```
Cargo.toml:
```
[dependencies]
serde = { version = "1.0.104", features = ["derive"] }
serde_json = "1.0.48"
```

## Trabalhando com Bancos de dados

É recomendado utilizar a biblioteca [Diesel](https://diesel.rs/guides/getting-started)
Consulte também [Postgresl](https://docs.rs/postgres/0.15.2/postgres/)
Consulte também [SQLite](https://docs.rs/sqlite/latest/sqlite/)
```Rust
```

Nesta seção eu dedico uma parte especial ao SQLite.
Em seguida eu recomendo você experimentar um banco de dados Profissional que suporta grande volume de dados como Postgresql
[Trabalhando com Banco de Dados]

Neste diretório eu reúno um estudo completo de SQL com Rust, incluindo discutindo alguns fundamentos de SQL [SQL completo com Rust]
# Log

In this example I explore the funcionalities of crate LOG, and how to show into console the status of server.
It's a continuation of example 3_api

## Criando Um Chat

[Acesse o Link](https://github.com/ricardodarocha/rust-chat-old)

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

###  TUI

This is how we can create amazing TUI Text User Interfaces with Rust.

[blue TUI](https://lib.rs/crates/cursive)
[term-table](https://lib.rs/crates/term-table)

A lot of other [TUI libraries](https://lib.rs/command-line-interface)

# Api 

Este é um guia para criar APIS com Rust
Clone o repositório e altere routes.rs
https://github.com/ricardodarocha/newapi

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

# Arquiteturas com RUST

Nesta seção eu reúno meus estudos mais recentes sobre Arquiteturas com Rust, inclusive comparando com outras linguagens e discutindo as vantanges e desvantanges de cada um. Eu convido fortemente a comunidade a participar desta discussão aqui [link]

# Links

**github notebook**(Best Rust curated)[https://github.com/brson/rust-anthology/blob/master/master-list.md]
**Video** (Error Handling)[https://www.youtube.com/watch?v=mhw-x5Q_-z0&t=195s]

https://lib.rs/crates/cursive
