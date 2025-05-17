---
marp: true
theme: default
class:
  - invert
---

# Introdução aos Algoritmos com Rust

![QR Code](qr.PNG)

https://github.com/ricardodarocha/Rust/blob/main/Curso/Rust.md

---

## Estrutura Básica de um Programa em Rust

Um programa em Rust sempre começa pela função main:

```rust
fn main() {
    println!("Olá, mundo!");
}
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=16d3ecbe55229cc374cadc3551d76d08)

---

## Exercício

`Imprima! "Olá {fulano}" | fulano = seu_nome`

---

## Variáveis

```rust
    let nome = "Aluno";
    println!("Olá, {aluno}", aluno = nome);
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ee38d196b4602936645bfdb004288e30)

---

## Tipos Primitivos

Rust possui diversos tipos de dados primitivos:

$\mathbb{N}$ Números naturais (u32, u64)
$\mathbb{Z}$ Números Inteiros (i32, i64)
$\mathbb{R}$ Real, Double (f32, f64)
Booleano (bool)
Caracteres (char)

```rust
let inteiro: i32 = 10;
let flutuante: f32 = 3.14;
let mol = 6.02e23;
let booleano: bool = true;
let caractere: char = 'A';
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1c3a984853e97042e9798c4020178fe5)

---

## Declaração de variáveis

```rust
let nome = "Santos Dummont";
let nascimento = 1873;
let idade = 2025 - nascimento;
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6b428dc2da0f8e98f7e5846e5e818f4e)

---

## Exercício 2

Faça o programa imprimir 14 bis. Percorra um fluxo de repetição de 1 a 14, incrementando um contador e imprima o número atual, 
até que na última iteração o programa imprima 14 bis

---

```rust
  for i in 1..=14 {
      imprima!("{} bis", i);
  }
  
```

---

## Variáveis mutáveis

```rust
let contador = 1; ❌
let mut contador = 1; ✅  
contador += 1;
```

---

## Tipo String

```rust
  let mut jogador: String;
    
  jogador = "Elfo".to_string();
  jogador = "Bruxo".to_string();
    
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=eef652370312d7b8bc4c96672bb60f8c)

---

## Blocos de decisão

```rust
if x == 42 {

}
```

---

## Exemplo

```rust
fn main() {
    let idade = 26;
    if idade > 18 {
        println!("Maior de idade");
    } else 
        println!("Menor de idade");
    }
```

---

## Fluxo de repetição

```rust
fn main() {
    let mut contador = 0;
    loop {
        if contador >= 5 {
            break;
        }
        println!("Contador: {}", contador);
        contador += 1;
    }
}
```

---

## For

```rust
fn main() {
    for i in 1..=5 {
        println!("Número: {}", i);
    }
}
```

---

## Matrizes Array

```rust
let alfabeto: [char; 3] = ['a','b','c'];
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=91d00c7a8e2654d4a0e234ad9dc7c724)
