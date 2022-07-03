# Um guia de Design Patterns com Rust

Neste repositório eu explico e mostro exemplos de design patter com RUST

## O Padrão Builder e o Dialeto Fluent

Utilizar a linguagem ubíqua é a forma mais recomendada para você escrever o ponto de entrada para a sua aplicação `main.rs` `lib.rs`

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

Este formato é possível na linguagem funcional se você escrever implementações como funções que retornam a se mesmo (Self)

```Rust
impl App() --> Self {
  //Your code;
  
  return Self
}
```

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
