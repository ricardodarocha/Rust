# Aula 1.1 O Compilador do Rust

## 1. Criando um novo projeto

Crie um projeto chamado **aula1**:

```bash
cargo new aula1
```

Acesse a pasta do projeto:

```bash
cd aula1
```

---

## 2. Executando o projeto

Compile e execute o programa padrão criado pelo Cargo:

```bash
cargo run
```

Saída esperada:

```text
Hello, world!
```

Explique que o comando `cargo run` executa duas etapas automaticamente:

1. Compila o projeto.
2. Executa o programa gerado.

---

## 3. Introduzindo um erro proposital

Substitua o conteúdo do arquivo `main.rs` por:

```rust
fn main() {
    let x = String::from("Rust");
    let y = x;

    println!("{x}");
}
```

Compile novamente:

```bash
cargo run
```

O compilador exibirá uma mensagem semelhante a:

```text
error[E0382]: borrow of moved value: `x`
```

Explique que:

* A variável `x` é dona da `String`.
* Ao executar `let y = x;`, a propriedade da `String` é transferida para `y`.
* Após a transferência (*move*), `x` deixa de ser válida.
* O compilador impede que um valor movido seja utilizado novamente.

---

## 4. Consultando a documentação do erro

O compilador informa o código do erro (`E0382`).

Para obter uma explicação detalhada, execute:

```bash
rustc --explain E0382
```

Mostre aos alunos que praticamente todos os erros do compilador possuem uma documentação própria com explicações e exemplos.

---

## 5. Corrigindo utilizando Borrowing

Em vez de transferir a propriedade, empreste uma referência:

```rust
fn main() {
    let x = String::from("Rust");

    let y = &x;

    println!("{x}");
    println!("{y}");
}
```

Agora o programa compila normalmente porque:

* `x` continua sendo a proprietária da `String`.
* `y` apenas possui uma referência para o mesmo valor.
* Nenhuma cópia da `String` foi realizada.

---

# 6. Imutabilidade

Por padrão, todas as variáveis em Rust são imutáveis.

```rust
fn main() {
    let idade = 20;

    idade = 21;

    println!("{idade}");
}
```

Erro:

```text
error[E0384]: cannot assign twice to immutable variable
```

**Correção:**

```rust
fn main() {
    let mut idade = 20;

    idade = 21;

    println!("{idade}");
}
```

---

# 7. Shadowing

O *shadowing* cria uma nova variável com o mesmo nome da anterior.

```rust
fn main() {
    let valor = 10;

    let valor = valor + 5;

    let valor = valor * 2;

    println!("{valor}");
}
```

Saída:

```text
30
```

Também é possível alterar o tipo da variável:

```rust
fn main() {
    let texto = "42";

    let texto: i32 = texto.parse().unwrap();

    println!("{texto}");
}
```

---

# 8. Ownership

Cada valor possui um único proprietário.

```rust
fn main() {
    let nome = String::from("Rust");

    let outro = nome;

    println!("{nome}");
}
```

Erro:

```text
error[E0382]: borrow of moved value
```

---

# 9. Borrowing

Em vez de mover, emprestamos uma referência.

```rust
fn main() {
    let nome = String::from("Rust");

    let outro = &nome;

    println!("{nome}");
    println!("{outro}");
}
```

---

# 10. Borrowing mutável

```rust
fn main() {
    let mut nome = String::from("Rust");

    let r = &mut nome;

    r.push_str(" Language");

    println!("{r}");
}
```

---

# 11. Dois empréstimos mutáveis

```rust
fn main() {
    let mut nome = String::from("Rust");

    let r1 = &mut nome;
    let r2 = &mut nome;

    println!("{r1}");
    println!("{r2}");
}
```

Erro:

```text
error[E0499]: cannot borrow as mutable more than once at a time
```

---

# 12. Empréstimo imutável e mutável

```rust
fn main() {
    let mut nome = String::from("Rust");

    let r1 = &nome;

    let r2 = &mut nome;

    println!("{r1}");
    println!("{r2}");
}
```

Erro:

```text
error[E0502]: cannot borrow as mutable because it is also borrowed as immutable
```

---

# 13. Vários empréstimos imutáveis

```rust
fn main() {
    let nome = String::from("Rust");

    let a = &nome;
    let b = &nome;
    let c = &nome;

    println!("{a}");
    println!("{b}");
    println!("{c}");
}
```

Esse código é permitido porque todas as referências apenas leem o valor.

---

# 14. Ownership em funções

```rust
fn imprime(texto: String) {
    println!("{texto}");
}

fn main() {
    let nome = String::from("Rust");

    imprime(nome);

    println!("{nome}");
}
```

Erro:

```text
error[E0382]: borrow of moved value
```

---

# 15. Borrowing em funções

```rust
fn imprime(texto: &String) {
    println!("{texto}");
}

fn main() {
    let nome = String::from("Rust");

    imprime(&nome);

    println!("{nome}");
}
```

# Exercícios

# Exercício 1 — Ownership com Variáveis

## Objetivo

Compreender como funciona a transferência de propriedade (*Ownership*) entre variáveis e como corrigir o problema utilizando *Borrowing*.

---

## Enunciado

1. Crie uma variável `x` do tipo `String`.
2. Transfira a propriedade para uma variável `y`.
3. Tente imprimir `x`.
4. Compile o programa e observe o erro.
5. Consulte a documentação do erro utilizando:

```bash
rustc --explain E0382
```

6. Corrija o programa utilizando uma referência (`&`).
7. Compile novamente e verifique que o programa executa corretamente.

---

# Solução

## Passo 1 — Código com erro

```rust
fn main() {
    let x = String::from("Rust");

    let y = x;

    println!("x = {}", x);
    println!("y = {}", y);
}
```

Ao compilar:

```bash
cargo run
```

O compilador exibirá uma mensagem semelhante a:

```text
error[E0382]: borrow of moved value: `x`
```

---

## Passo 2 — Entendendo o erro

Quando executamos:

```rust
let y = x;
```

A propriedade da `String` é transferida para `y`.

Após essa linha:

* ❌ `x` deixa de ser válida.
* ✅ `y` passa a ser a única proprietária da `String`.

Por isso, o compilador impede o acesso a `x`.

---

## Passo 3 — Consultando a documentação

Execute:

```bash
rustc --explain E0382
```

A documentação explica que o erro ocorre quando tentamos utilizar um valor cuja propriedade já foi movida para outra variável.

---

## Passo 4 — Corrigindo com Borrowing

```rust
fn main() {
    let x = String::from("Rust");

    let y = &x;

    println!("x = {}", x);
    println!("y = {}", y);
}
```

Saída:

```text
x = Rust
y = Rust
```

---

## O que mudou?

Antes:

```rust
let y = x;
```

A propriedade era transferida para `y`.

Agora:

```rust
let y = &x;
```

`y` recebe apenas uma **referência** para a `String`.

Assim:

* `x` continua sendo a proprietária.
* `y` apenas empresta o valor.
* Nenhuma cópia da `String` é realizada.

---

## Conclusão

Este exercício demonstra uma das principais garantias de segurança do Rust: **um valor possui apenas um proprietário por vez**. Quando precisamos apenas acessar um valor sem assumir sua propriedade, utilizamos **Borrowing** por meio de referências (`&`). Dessa forma, o compilador garante segurança de memória sem necessidade de coleta de lixo (*Garbage Collector*).


Nesse caso, a função apenas recebe uma referência, mantendo a propriedade da `String` com a função `main`.
