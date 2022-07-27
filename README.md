# Rust

Um repositório para estudar Rust do começo  

🚀 **Este repositório está crescendo rápido**  

- [Configurar um Ambiente Rust](https://www.rust-lang.org/pt-BR/tools/install)  
- [Hello World](#primeiro-exercício)  
- [Api](https://github.com/ricardodarocha/Rust/blob/main/3_api/Readme.md)  
- [Postgres](https://github.com/ricardodarocha/sqlxpg)  
---

Eu tenho um interesse especial por **Rust!** Este material começou a ser criado para atender ao meu objetivo pessoal de aprender Rust.    
Com o passar do tempo, eu fui percebendo que isso poderia ajudar outras pessoas. Desde então, eu tenho me esforçado para deixar tudo organizado.

Eu criei o formulário abaixo para examinar os interesses da comunidade.  
Por favor, responda a algumas perguntas bem rapidinho.
https://forms.gle/g5Y6V3g5ag75qcPg8  

# Introdução

Se você não tem conhecimento de qualquer linguagem de programação, é recomendado aprender os fundamentos primeiro.   
Aprenda **Python** ou **C** para compreender como as linguagens de programação funcionam.  
**Rust** é uma linguagem excelente, mas possui vários recursos avançados, e não é recomedável que você comece por ela. É recomendável que você aprenda **Rust** como segunda linguagem até se tornar experiente.  

Veja minha [trilha de python](https://github.com/ricardodarocha/Python)

Você pode facilmente aprender **Rust** na internet, lendo o [Livro de Rust Oficial](https://livro.rustbr.org/), acessando a [Documentação](https://www.rust-lang.org/pt-BR/). Há muitos cursos na internet mas o que eu recomendo é você fazer parte da comunidade de desenvolvedores. Cadastre-se em fóruns e grupos colaborativos, monte um repositório e compartilhe com os colegas. E claro, se inscreva nos canais de Rust no Brasil e no Exterior e fique por dentro de tudo que acontece neste universo maravilhoso do Rust.

Para exercitar o que você vai aprender, primeiro configure um ambiente. 
[Diversas formas de configurar um ambiente Rust](https://www.rust-lang.org/pt-BR/tools/install)

ou brinque no [Rust Playground](https://play.rust-lang.org/) antes de configurar um ambiente local 🎠

# Primeiro exercício

A forma mais comum de aprender uma linguagem é começar pelo exemplo _Olá mundo_.  

```Rust
fn main() {
    println!("Olá, mundo!");
}
```
[tente agora on-line!](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=58b8666302b1dfb3863d0de5e17286ee)

Há algumas variações deste programa que eu explico aqui, com o objetivo de incrementalmente ir aprendendo novos recursos. Vou utilizar comentários no código para explicar. Veja o tutorial sobre [Como comentar códigos Rust](https://github.com/ricardodarocha/Rust/edit/main/Documentando.md).

```Rust
// Disponibilizando diferentes versões para o seu usuário
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    // neste exemplo, além de aprender a exibir a versão você também aprendeu como formatar valores na mensagem
    println!("Olá, mundo! \nEsta é a versão {}", VERSION);
}
```

# 🦀 Dica

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

[match](https://github.com/ricardodarocha/Rust/new/main/general/correspondencia.rs)  

### módulos de estudo

[parte 1](#parte-i)  
[parte 2](#parte-ii)  
[parte 3](#parte-iii)  

### Índice remissivo

# A

Actix  
[Ambiente, configurando](https://www.rust-lang.org/pt-BR/tools/install)  
Ambiente, variáveis de (ver env)  
Argumentos, Args  
[Api](https://github.com/ricardodarocha/Rust/blob/main/3_api/Readme.md)  
[Arquivos](#ler-arquivos)  
[Arquitetura](https://github.com/ricardodarocha/Rust/tree/main/arquitetura)  
Aplicativos de Linha de Comando (ver CLI)  
Aplicativos Desktop (ver Desktop)
Aplicativos Web (ver Web)  
Arduino  
ASM  
Atributos (ver Models)  


# B

[Banco de Dados](#trabalhando-com-bancos-de-dados)  
BI  
Binários, Bin  
Box  

# C

[Cargo](#cargo)  
[Cargo.toml](#cargotoml)  
[Chat](https://github.com/ricardodarocha/rust-chat)   
CLAP  
[Configurações do usuário](#ler-um-arquivo-de-configurações-env-ini-xml-json)  
[CLI](#criando-uma-aplicação-simples-de-linha-de-comando-cli)  
Cores  
CSV  
 
# D

Dashboard  
DDD  
[Debug](#debug)  
[Default](https://github.com/ricardodarocha/Rust/blob/main/general/default.md)  
[Display](https://github.com/ricardodarocha/Rust/blob/main/general/display.md)  
[Design Patterns](https://github.com/ricardodarocha/Rust/tree/main/DesignPattern)  
[Derive](#derive)  
Desktop  
Documentação, Doc    
DOD  

# E

E-mail  
Entidades (ver Models)  
Enumerados, Enum 
env, .env    
Erros  
Estatística  
Estruturas de Dados (ver Struct)  

# F

Firebird  

# G

Games (ver Jogos)  
Gráficos  
Grafos  
GUI  

# H

Hello world  
HTML  

# I
Imagens 
[Impl Struct](#impl-struct)  
[Impl Trait](#impl-trait)  
[Ini, \*.ini](#ler-um-arquivo-ini)  
IP  
Internet  
Instaladores  

# J

Jogos  
[Json](#lendo-json)  

# L

Linguagem Ubíqua  
Log  

# M

Macros  
Matrizes  
[Models](https://github.com/ricardodarocha/Rust/blob/main/Modelos.md), [Modelos](https://github.com/ricardodarocha/Rust/blob/main/Modelos.md)  
Módulos  

# P

Parâmetros (ver args)  
Playground  
POO  
Postgres  

# R

Result  
Return  

# S

SQL  
[Struct](#struct)  
[Strings](#string)  

# T

Tauri  
Testes
Texto  
Trait ver [Impl Trait](#impl-trait)  
Tipos  
Tuplas  

# V

Variáveis
Variáveis de ambiente (ver env)  
Vetores  
Vue.js  

# X  

Xadrez  
XML  

🚧 Nesta seção eu organizo os conteúdos por ordem alfabética. Se preferir uma sequência de estudos acompanhe o tutorial a seguir.

# Parte I  

A sequência de estudo que eu tenho aprimorado para você que está começando do zero é a seguinte. E eu vou tentar explicar a razão disso:

## Cargo  

### Lista de comandos do compilador

```shell
cargo check #verifica se todos os pacotes estão instalados
cargo build #compila o binário DEBUG
cargo run #roda o binário DEBUG
cargo clippy #verifica a qualidade semântica do algoritmo 
cargo run --release #roda o binário RELEASE
```
## Cargo.toml

### Alterando as informações de propriedade intelectual

Acesse o cargo.toml e edite os atributos da aplicação com as suas informações

```
description — Uma descrição do programa  
documentation — O site onde está publicado o pacote rustdoc.rs  
readme — Um arquivo Markdown local sobre a sua aplicação  
homepage — O site da aplicação  
repository — O repositório Github onde está publicado o código fonte  
license — Tipo de licença  
license-file — Os termos da licença (txt)  
keywords — Palavras chave do projeto  
categories — Categoria do pacote  
```

[Lista completa de atributos](https://doc.rust-lang.org/cargo/reference/manifest.html)  

## Ler um arquivo de configurações (.env .ini .xml .json)

Para gerenciar as configurações do usuário escolha uma extensão de arquivo
Há libs específicas para cada formato de arquivo, que eu vou mostrar a seguir

### Informações do binário

O arquivo **cargo.toml** possui informações básicas do binário

_cargo.toml_
```Rust
[package]
name = "my_package"
version = "0.0.1"
repository = "https://github.com/rust-lang/my-package/"
license = "MIT OR Apache-2.0"
authors = "Ricardo da Rocha"

[dependencies]
```

Acesse estas informações utilizando constantes estáticas  

_main.rs_
```Rust
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
  println!("Hello, world! \nThis is version {} \nThanks to {}", VERSION, AUTHORS);
  }
```

### Ler um arquivo ini

[doc](https://github.com/mexili/ini-rs)


### .env variáveis de ambiente

Acesse as variáveis de ambiente, ou configure o ambiente utilizando um arquivo **.env**  
 
src/.env
```Rust
/// ```ini
/// user_name=admin
/// ```
use dotenv;
dotenv().expect(".env file not found");
println!("KEY", env::var("user_name").unwrap());
env::set_var("password", "unbush84likely8Fdetail42");
```

### passando parâmetros para a aplicação com args()

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

## Ler arquivos 

> 🧡 _Veja esse hack_ [Carregar o conteúdo de um arquivo para uma string](#carregar-arquivo-em-string)

Da mesma forma que criamos arquivos, é necessário ler os dados gravados.

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

### Carregar arquivo em string

```Rust
use std::fs;
use std::path::Path;

let conteudo = fs::read_to_string(Path::new("./Sample.txt") );
match conteudo {
  some(text) => print!(text),
  None => print!("Não encontrado")
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

# Parte II

# 🦀

Boas práticas de desenvolvimento e dicas de projeto  

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

## Outras lições da linguagem com este exemplo básico

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

# Parte III

## Tipos

[Tipos primitivos](https://doc.rust-lang.org/rust-by-example/primitives.html)  
[Tipos customizados](https://doc.rust-lang.org/rust-by-example/custom_types.html)  
[Strings](https://doc.rust-lang.org/rust-by-example/conversion/string.html)  

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

## Gerando uma senha

```Rust
//Create random passwords from a set of alphanumeric characters
//rand-badge cat-os-badge

//Randomly generates a string of given length ASCII characters in the range A-Z, a-z, 0-9, with Alphanumeric sample.


use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
//Create random passwords from a set of user-defined characters
//rand-badge cat-os-badge

//Randomly generates a string of given length ASCII characters with custom user-defined bytestring, with gen_range.


fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
```
[fonte](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html)   

## Trabalhando com Bancos de dados

É recomendado utilizar a biblioteca [Diesel](https://diesel.rs/guides/getting-started) ou [sqlx]()  
Consulte também [Postgresl](https://docs.rs/postgres/0.15.2/postgres/)
Consulte também [SQLite](https://docs.rs/sqlite/latest/sqlite/)
Consulte também [Firebird](https://github.com/fernandobatels/rsfbclient)  
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

## Box  

Crie referências encadeadas com Box  
Veja este exemplo de árvore genealógica  
```Rust
#[derive(Debug)]
struct MinhaArvore {
    nome: String,
    ramos: Option<Box<MinhaArvore>>,
}

fn main() {
    let galileu = MinhaArvore {
        nome: "Galileu",
        ramos: Some(Box::new(MinhaArvore{
            nome: "Newton",
            ramos: Some(Box::new(MinhaArvore){
                nome: "Einstein",
                ramos: Some(Box::new(MinhaArvore))
                })
            })
        )
    }
}

println!("{#?}", galileu);
```

## Debug  

Debug pode se referir a 
[target de compilação]()   
[derive annotation]()  
[processo de depuração]()  

### target debug

Há dois tipos de binários *DEBUG* e *RELEASE*
Ao compilar um projeto com `cargo build` é criado um binário na pasta `\target\debug\`. Este binário normalmente é um pouco maior do que uma versão release  
A versão release é utilizada para distribuição do seu aplicativo, e a versão debug é utilizada para testes  
A versão debug normalmente contém mais informações importantes para o desenvolvedor testar e procurar por BUGS    
Para compilar uma versão debug basta rodar o comando `cargo run` e a versão relase `cargo run --release`

```shell
cargo run 
// \target\debug\projeto.exe

cargo run --release
// \target\release\projeto.exe
```

## derive

Esta é mais uma macro do RUST que nos auxilia a gerar instruções sem a necessidade de escrever código  Rust irá gerar automaticamente várias linhas de código quando utilizarmos derive  
Há algumas anotações específicas com objetivos específicos  
Veja na tabela a seguir alguns exemplos de derive em estruturas de dados [Struct](#struct)  

||||
|-|-|-|
|`#[derive(Debug)]`|permite imprimir informações em runtime|`println!("{#?}", objeto)`|
|`#[derive(Default)]`|permite atribuir valores default ao criar uma nova instância|`Objeto::defaul()`|
|`#[derive(Default)]`|permite representar o objeto em string|`Objeto::to_str()`|
|`#[derive(Serialize)]`|permite converter em Json|``|
|`#[derive(Deserialize)]`|permite ler a partir de um Json|``|
|`#[derive(Clone)]`|permite criar cópias do objeto|`obj2 = obj1.clone()`|

```

```

# Struct

Estruturas de dados representam objetos, embora não seja exatamente uma linguagem POO é possível criar objetos com Rust utilizando Structs. A diferença é que RUST não possui suporte a herança. No entanto vários problemas de herança podem ser resolvidos utilizando Composition  

```Rust
struct MeuObjeto {
  id: Uuid,
  nome: String,
  quantidade: i32, //-♾ ..2^32
  celulas: u16, //0..255
  ativo: bool,
  arquivo: Path
}
```
Observe que Rust possui vários tipos primitivos mas também possui tipos enriquecidos, com Path, Filename, Date etc
Para isso é necessário importar crates externas

```Rust
use std::path::Path;
use uuid::Uuid;
```

## Advanced Struct - Estruturas avançadas

Estruturas normalmente representam dados, no entanto é possível adicionar métodos de comportamentos aos objetos utilizando impl

## Impl Struct

Implementar comportamento em estruturas

```Rust
struct MeuObjeto {
  id: Uuid,
  nome: String,}
  
  impl MeuObjeto {
  
  fn New() -> &Self {
    MeuObjeto {
      id: Uuid::new_v4(),
      nome: "Anônimo".to_own(),
    }
  }
  }
``` 

## Impl Trait

Implementando contratos em estruturas

Além de implementar métodos é possível implementar contratos   
Contratos são similares à interfaces. Com a diferença de que interfaces podem ou não implementar heranças e possuem características inerentes à POO, traits possuem particularidades que deixam-no mais poderosos.

por exemplo vamos implementar um contrato Admin na nossa estrutura

```Rust
pub trait Admin {
   fn login(&self) -> bool {
     false
   };
}
```
Veja que tratis possuem uma implementação padrão, ou seja não exige abstração em cenários onde não é necessário

```Rust
impl Admin for MeuObjeto {
  fn login(&self) -> bool {
     return nome == "Admin"
   };
}

```

Neste exemplo o objeto que possuir o nome "Admin" irá retornar Login=True

```Rust

let user1 = MeuObjeto::new();
let user2 = MeuObjeto::new(); 
let user2.nome = "Admin".to_own();

print!("user 1 can login {}", user1.login()) //false
print!("user 2 can login {}", user2.login()) //true
}
```

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

Este é um guia para criar [Api com Rust](https://github.com/ricardodarocha/newapi)  

Para criar apis avançadas com acesso ao banco de dados acesse
[Api Postgres](https://github.com/ricardodarocha/sqlxpg)

## Api básica

Neste exemplo eu mostro como criar uma API do zero  
Clone o repositório e altere routes.rs

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

Nesta seção eu reúno meus estudos mais recentes sobre Arquiteturas com Rust, inclusive comparando com outras linguagens e discutindo as vantanges e desvantanges de cada um. [link](https://github.com/ricardodarocha/Rust/tree/main/arquitetura)

# Links

**github notebook**(Best Rust curated)[https://github.com/brson/rust-anthology/blob/master/master-list.md]
**video** (Error Handling)[https://www.youtube.com/watch?v=mhw-x5Q_-z0&t=195s]  
(**rust cookbook**)[https://rust-lang-nursery.github.io/rust-cookbook]
https://lib.rs/crates/cursive
