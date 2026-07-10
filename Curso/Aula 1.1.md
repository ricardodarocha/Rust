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

# Lista de Exercícios — Rust: Ownership, Borrowing, Funções e Imutabilidade

## Exercício 1 — Ownership com Variáveis

### Objetivo

Compreender como funciona a transferência de propriedade (**Ownership**) entre variáveis e como corrigir o problema utilizando **Borrowing**.

---

## Parte 1 — Gerando o erro

### Enunciado

1. Crie uma variável `x` do tipo `String`.
2. Transfira a propriedade para uma variável `y`.
3. Tente imprimir `x`.
4. Compile o programa e observe o erro gerado pelo compilador.

### Código

```rust
fn main() {
    let x = String::from("Rust");

    let y = x;

    println!("x = {}", x);
    println!("y = {}", y);
}
```

### O que aconteceu?

Ao executar `let y = x;`, a propriedade da `String` é transferida para `y`.

Como `String` possui um tamanho variável armazenado na heap, apenas um proprietário pode existir por vez.

Após a transferência, `x` deixa de ser válida e o compilador impede seu uso.

---

## Parte 2 — Corrigindo com Borrowing

### Enunciado

Altere o código para que `y` receba apenas uma referência para `x`, sem transferir a propriedade.

### Código

```rust
fn main() {
    let x = String::from("Rust");

    let y = &x;

    println!("x = {}", x);
    println!("y = {}", y);
}
```

### Saída esperada

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

A propriedade da `String` era transferida para `y`.

Agora:

```rust
let y = &x;
```

`y` recebe apenas uma referência para a `String`.

Consequentemente:

- `x` continua sendo a proprietária da `String`;
- `y` apenas empresta o valor;
- nenhuma cópia da `String` é realizada.

---

# Exercício 2 — Calculando a idade de Santos Dumont

## Objetivo

Praticar declaração de variáveis, operações aritméticas e entrada de dados.

### Enunciado

Crie um programa que calcule a idade de **Santos Dumont**.

### Requisitos

- Crie uma constante contendo o ano de nascimento de Santos Dumont.
- Solicite ao usuário o ano atual.
- Calcule e exiba a idade.

> **Dica:** Santos Dumont nasceu em **1873**.

### Exemplo

```
Ano atual: 2026

Idade de Santos Dumont: 153 anos
```

```rust
fn main() {
    let ano_atual = 2026;
    let idade = ano_atual - 1873;
}
```

---

# Exercício 3 — Validação de valores

## Objetivo

Praticar validação de dados.

### Enunciado

Altere o programa anterior para impedir valores inválidos.

### Regras

O programa não deve aceitar:

- ano negativo;
- idade negativa.


### Exemplo

```rust
fn main() {
    let ano_atual: u16 = 2026;
    let idade: u16 = ano_atual - 1873;
    let idade: u16 = 0 - 1873; ❌ erro
}
```

```
Ano atual: -10

Erro: o ano não pode ser negativo.
```

---

# Exercício 4 — Refatoração utilizando funções

## Objetivo

Separar responsabilidades do programa.

### Enunciado

Extraia o cálculo da idade para uma função.

A função deve possuir a seguinte assinatura:

```rust
fn calcular_idade(ano_nascimento: u16) -> u16 {
    let ano_atual: u16 = 2026;
    let idade = ano_atual - ano_nascimento;
    return idade;
}
```

Depois, utilize essa função dentro do `main()`.

### Exemplo

```rust
let idade = calcular_idade(1873);
```

> Dica: Dentro de uma função não é obrigatório usar a palavra return,
> para retornar basta omitir o ponto e vírgula da última linha da função
```rust 
fn calcular_idade(ano_nascimento: u16) -> u16 {
    2026 - ano_nascimento
}
```

---

# Exercício 5 — Borrowing com múltiplas referências

## Objetivo

Compreender que diversas referências imutáveis podem coexistir.

### Enunciado

1. Crie uma variável:

```rust
let nome = String::from("Santos Dumont");
```

2. Crie quatro referências para essa variável:

```rust
let a = &nome;
let b = &nome;
let c = &nome;
let d = &nome;
```

3. Imprima todas as variáveis.

### Resultado esperado

```text
nome = Santos Dumont
a = Santos Dumont
b = Santos Dumont
c = Santos Dumont
d = Santos Dumont
```

---

# Exercício 6 — Imutabilidade e mutabilidade

## Objetivo

Compreender como declarar variáveis mutáveis.

### Enunciado

Crie uma variável mutável chamada `nome`.

Inicialmente ela deve armazenar:

```
Santos Dumont
```

Imprima seu conteúdo.

Depois altere o valor para:

```
Carlos Drummond
```

```rust
let mut nome = "Santos Dummont";
println!("{}", nome);
nome = "Carlos Drummond";
println!("{}", nome);
```

### Exercício 6.1

> Dica: Sempre que possível, troque o modo mut por shadowing

```rust
let nome = "Santos Dummont";
println!("{}", nome);
let nome = "Carlos Drummond";
println!("{}", nome);
```

### Exemplo

```text
Nome: Santos Dumont
Nome: Carlos Drummond
```

---

# Exercício 7 — Funções retornando tuplas

## Objetivo

Aprender a retornar múltiplos valores utilizando tuplas.

---

## Parte A — Cores da seleção

Crie uma função que retorne uma tupla contendo as cores da seleção brasileira.

Exemplo de assinatura:

```rust
fn cores_selecao() -> (&String, &String) {

}
```

Resultado esperado:

```text
Verde 
Azul
```
> Dica: Aprenda a ler os erros do compilador, o código acima não compila porque tem dois problemas
> 1. Aprenda a corrigir o lifetime, quando o compilador pedir, adicione 'static
> 2. Substitua String por str, ou coloque .to_string()

```rust
(&String, &String) {
  |                        ^        ^ expected named lifetime parameter
  |                        |
  |                        expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
3 | fn cores_selecao() -> (&'static String, &'static String) {
  |                         +++++++          +++++++
```

**Resposta correta**

```rust
fn cores_selecao() -> (&'static str, &'static str) {
    ("Amarelo", "Azul")
}
 
fn main() {

    let (a, b) = cores_selecao();
    println!("{a} {b}");
}
```
---

## Parte B — Retornando duas referências

Crie uma função que retorne duas referências para `&str`.

Exemplo:

Entrada lógica:

```
"Santos Dumont"
```

Retorno:

```rust
("Santos", "Dumont")
```

Assinatura sugerida:

```rust
fn nome_sobrenome() -> (&'static str, &'static str) {

}
```

Saída esperada:

```text
Nome: Santos
Sobrenome: Dumont
```

**resposta correta**
```rust
fn nome_sobrenome() -> (&'static str, &'static str) {
    let nome = "Santos Dummont";
    (&nome[0..6], &nome[7..14])
}
 
```
---

### Avançado

Técnica de desestruturação 
Acesse os valores de uma tupla por meio de desestruturação

```rust
fn main() {

    let (a, b) = nome_sobrenome();
    println!("{a} {b}");
}
```
