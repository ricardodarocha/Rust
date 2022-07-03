# Um guia de Design Patterns com Rust

Neste repositório eu explico e mostro exemplos de design patter com RUST

## O Padrão Builder e o Dialeto Fluent

💬 Utilizar a linguagem ubíqua é a forma mais recomendada para você escrever o ponto de entrada para a sua aplicação `main.rs` `lib.rs`

```Rust
fn main() {
  HttpServer::new( move || {
        let logger = Logger::default();
        App::new()
        .data( pool.clone() )
        .wrap(logger)
        .service(ping)
        .service(pong)
}
```

Este formato é possível na linguagem funcional se você escrever implementações como funções que retornam a si mesmo (Self)

✨ **Rust possui suporte nativo** ao padrão Builder por meio da macro **derive_builder**


```Rust
#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder)]
#[builder(setter(build))]
struct pub struct Server {
  host: String,
  port: u16,
  timeout: Option<ms>,
  }
}
```
✨ Esta simples expansão irá permitir construir uma nova instância de Server assim

```Rust
let serv = ServerBuilder::default()
.host("localhost".to_owned),
.port(9090),
.timeout(3000):
.build()
.unwrap();
```

👶 No entanto se você é iniciante eu recomendo implementar este design patter com as suas próprias mãos:

```Rust
impl App{
  fn atribuir(&mut self, valor: any) --> &mut Self {
  self.any = any;  
  return self
}
```

🎯 Idiomaticamente é recomendável escrever estruturas com responsabilidades específicas, criando um struct específico para construir (Builder) uma estrutura Comportamental (Business)

```Rust
pub struct Server {
  host: String,
  port: u16,
  timeout: Option<ms>,
  }
  
  impl Server {
    fn index() -> HttpResponse {
    return Ok('<div> Olá Mundo </div>'.to_owned())
    }
    
 -------------
 
pub struct ServerBuilder {
  host: String,
  port: u16,
  timeout: Option<ms>,
  }
 

  impl ServerBuilder {
    fn host(&mut self, host: String) -> &mut Self {
      self.host = host;
      return self
    }
    fn port(&mut self, port: u16) -> &mut Self {
      self.port = port;
      return self
    }
    fn timeout(&mut self, value: i32) -> &mut Self {
      self.timeout = ms(value);
      return self
    }
    fn build(&mut self2) -> Server {
      return Server {self.host, self.port, self.timeout}
    }
}

```

https://www.youtube.com/watch?v=5DWU-56mjmg&t=419s _em inglês_

## Iterator

Iterar é uma das habilidades mais importantes das linguagens de programação
Vamos revisar as estruturas de looping disponíveis pela linguagem Rust

```
loop {
};

for _ in 0..10 { }
```

o _Design Pattern_ **iterator** é uma expansão deste conceito que utiliza alguns recursos mais avançados para ampliar as possibilidades de programação, a segurança e a performance.
Observe o código Rust para separar uma string simples

```Rust
let todos_meses_do_ano = "Janeiro, Fevereiro, Março, Abril, Maio, Junho, Julho, Agosto, Setembro, Outubro, Novembro, Dezembro";
let meses: Vec<&str> = 
todos_meses_do_ano.split(',')
 .map(str::trim)
 .collect();
 ```
 
 Observe que o método map é chamado para cada elemento do iterador _split_  
Split é um iterador porque implementa o `trait Iterator` que possui a seguinte estrutura

```Rust
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}
```

Saiba mais em https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/iterator.rs

## Observer

Observer utiliza uma estrutura de Notificadores e Assinantes  
Um Notificador é um detentor de uma informação ou um Agente capaz de informar um ou mais assinante quando houver uma novidade, ou após a modificação do estado de uma aplicação.

Podemos exemplificar o Observer como um banco de dados REDIS que irá disparar uma notificação para as aplicações CLIENT quando for inserido um novo item no banco de dados, por exemplo. No entanto este design pattern será implementado inteiramente numa mesma camada da aplicação.

### Caso prático

Imagine que uma aplicação que monitore o preço das ações na Bolsa de Valores  
Sempre que uma ação da Bolsa de valores cair ou subir, irá notificar os clientes que possuem aquela ação na sua carteira.  
No entanto para não ter problemas de tráfego, a aplicação irá notificar apenas as 100 ações com alterações mais relevantes

A representação deste aplicativo ficaria assim

```Rust
impl Notificador {

fn Notificar(assinantes Vec<Assinante>) {
for acao in acoes
  assinantes(acao).notify(acao)
  }

fn Assinar(assinante, acao) {
  Self.assinantes(acao).push(assinante)
  }
}
```

```Rust
impl Assinante {

fn Assinar(notificador: Notificador; acoes: Acao)
for acao in acoes
  notificador.assinar(Self, acao)
  }
```


```Rust
use std::rc::Weak;

struct Event;

trait Observable {
    fn register(&mut self, observer: Weak<dyn Observer>);
}

trait Observer {
    fn notify(&self, event: &Event);
}
```
