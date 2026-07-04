# PARTE II Estruturas de Dados em Rust

*Material de apoio – Curso de Algoritmos e Estruturas de Dados com Rust*
UNIFAGOC – Computação
Autor: Ricardo da Rocha Vitor

---

## 📌 Introdução

Em Rust, estruturas de dados são construídas com base em um sistema de tipos forte, seguro e expressivo. A linguagem favorece **imutabilidade por padrão**, composição de tipos e uso explícito de abstrações como `struct`, `enum`, `trait` e módulos.

Este material aborda os principais blocos fundamentais para modelagem de dados em Rust.

---

# 1. Variáveis

Variáveis em Rust são declaradas com `let` e são **imutáveis por padrão**.

```rust
let x = 10;
```

Para permitir alteração:

```rust
let mut x = 10;
x += 1;
```

### Tipagem explícita

```rust
let idade: i32 = 20;
let pi: f64 = 3.14;
```

### Tipagem implícita

```rust
let idade = 20;
let pi = 3.14;
```

---

# 2. Tuplas

Tuplas agrupam valores de diferentes tipos.

```rust
let pessoa: (&str, i32) = ("Ana", 25);
```

Tuplas também podem ser usadas para declarar structs sem atributos nomeados

```rust
type Cor = u8; //um byte para representar uma cor 🟥🟩🟦
struct RGB(Cor, Cor, Cor);

let azul = (0, 0, 0xFF); 
```

### Acesso por índice

Os valores da tupla podem ser acessadas por índice
```rust
println!("{}", cor.0);
println!("{}", cor.1);
println!("{}", cor.2);
```

### Desestruturação

Tanto tuplas quanto structs podem ser acessadas por desestruturação das variáveis

```rust
let (nome, idade) = pessoa;
let RGB(r,g,b) = azul;
```

---

# 3. Structs

Structs representam estruturas nomeadas de dados.

```rust
struct Pessoa {
    nome: String,
    idade: u32,
}
```

### Instanciação

```rust
let p = Pessoa {
    nome: String::from("Carlos"),
    idade: 30,
};
```

### Acesso aos campos

```rust
println!("{}", p.nome);
```

---

## 🔧 Struct com métodos

```rust
impl Pessoa {
    fn nova(nome: String, idade: u32) -> Self {
        Self { nome, idade }
    }

    fn apresentar(&self) {
        println!("{} tem {} anos", self.nome, self.idade);
    }
}
```

---

# 4. Enumerados (enum)

Enums representam tipos com múltiplos estados possíveis.

```rust
enum Estado {
    Ligado,
    Desligado,
}
```

### Enum com dados

```rust
enum Resultado {
    Sucesso(i32),
    Erro(String),
}
```

### Pattern Matching

```rust
match resultado {
    Resultado::Sucesso(valor) => println!("{}", valor),
    Resultado::Erro(msg) => println!("{}", msg),
}
```

---

# 5. Módulos em Rust

Módulos organizam código e controlam visibilidade.

## 📁 mod

```rust
mod matematica {
    pub fn soma(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

## 🔓 pub mod

Torna o módulo público (em arquivos separados ou internos).

```rust
pub mod io_utils;
```

---

## 📥 use

Importa funções, structs ou módulos.

```rust
use crate::matematica::soma;
```

---

## 🧱 Estrutura de projeto

```
src/
 ├── main.rs
 ├── matematica.rs
 └── io_utils.rs
```

---

# 6. Cargo e dependências

Adicionar bibliotecas ao projeto:

```bash
cargo add csv
```

Exemplo de uso da crate:

```rust
use csv::Reader;
```

---

# 📌 Apêndice

## 🔹 `impl` (implementação)

Permite adicionar comportamento a structs e enums.

```rust
impl Pessoa {
    fn idade_dobro(&self) -> u32 {
        self.idade * 2
    }
}
```

---

## 🔹 Traits

Traits definem comportamentos compartilhados (interfaces).

```rust
trait Falar {
    fn falar(&self);
}
```

### Implementação de trait

```rust
impl Falar for Pessoa {
    fn falar(&self) {
        println!("Olá, meu nome é {}", self.nome);
    }
}
```

---

## 🔹 Trait como abstração

* Similar a interfaces em outras linguagens
* Permite polimorfismo em tempo de compilação

---

# 🚀 Conclusão

Rust fornece um modelo poderoso e seguro para construção de estruturas de dados através de:

* Structs para modelagem
* Enums para estados
* Traits para abstração
* Módulos para organização
* Sistema de tipos forte para segurança

Esses conceitos formam a base para estruturas avançadas como árvores, grafos, listas e sistemas complexos.

---

Se quiser, posso expandir este README para incluir:

* 📊 Listas encadeadas em Rust
* 🌳 Árvores binárias
* 🕸 Grafos com `petgraph`
* ⚙ Implementação completa de ADTs
* 🧠 Exercícios estilo prova UNIFAGOC

Só dizer o próximo nível.
