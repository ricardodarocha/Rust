---
marp: true
theme: default
class:
  - invert
---

## Funções

```rust
fn soma(a, b) {
    a + b
}
```

```rust
fn divide(a, b) {
    if b > 0 {
        a / b
    }
}

---

## Soma

```rust
fn soma(a: int, b: int) -> int {
    a + b
}
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6a4bb1dc15efc19082674847cf3dbaa8)

## Divisão

🔥
```rust
fn divide(a: int, b: int) -> int {
    if b > 0 {
        a / b  
    } else
    { 
        panic!("Divisão por zero") 
    }
}
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab7602fa760acff77c12879c79da003b)



---

## Exercício

Crie uma função para calcular o triplo de um número $n$ qualquer   
Calcule e imprima o triplo de 4 números $[2 3 5 8]$

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6b1a4a24ead017a023f51a8095c351cc)

## Crie uma função para verificar o maior número 

série de teste $$32 65 17 11 41$$

```rust
fn maior(a: int, b: int) -> int {
    if a > b {
        a
    } else
    { 
        b
    }
}
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a6be006b9caa7c26c4564a4d5728dd9c)

## Funções de teste

#[test]
fn testar() {
    assert!(2 + 2 == 4);
}

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4c7fe0134fa7efdcdc27779b7c1e2d8d)

---

## Programação orientada a testes

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ae6d1f9fe5fe2b9df5966d911eba01b1)

## Exercício final

Crie um programa que calcule o valor do Imposto de Renda a descontar de um funcionário com salário $s$ qualquer

| Faixa salarial (mensal)          | Alíquota (%) |
| -------------------------------- | ------------ |
| Até R\$ 2.000,00                 | Isento       |
| De R\$ 2.000,01 até R\$ 2.800,00 | 7,5%         |
| De R\$ 2.800,01 até R\$ 3.700,00 | 15%          |
| De R\$ 3.700,01 até R\$ 4.500,00 | 22,5%        |
| Acima de R\$ 4.500,00            | 27,5%        |

## Exercício final

Crie um programa que calcule o valor do Inss a descontar de um funcionário com salário $s$ qualquer

| Faixa Salarial (mensal)          | Alíquota (%) |
| -------------------------------- | ------------ |
| Até R\$ 1.500,00                 | 7,5%         |
| De R\$ 1.500,01 até R\$ 2.500,00 | 9,0%         |
| De R\$ 2.500,01 até R\$ 3.500,00 | 12,0%        |
| Acima de R\$ 3.500,00            | 14,0%        |

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3cf070802057e0e891452e92410a6ffd)