---
marp: true
theme: default
class:
  - invert
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

## Tabela verdade

Gere a tabela verdade do AND e do OR para os valores V e F
$Sejam a, b variáveis lógicas$
$|$ or
$&$ and


```rust
println!("Tabela Verdade do AND");
for a in ['V', 'F'] {
    for b in ['V', 'F'] {
        if (a=='V') & (b=='V') {
            println!("{} e {} | {}", a, b, "V")
        } 
        else {
            println!("{} e {} | {}", a, b, "F")
        }
    }
}

```

---
```rust
println!("Tabela Verdade do OR");
for a in ['V', 'F'] {
    for b in ['V', 'F'] {
        if (a=='V') | (b=='V') {
            println!("{} ou {} | {}", a, b, "V")
        } 
        else {
            println!("{} ou {} | {}", a, b, "F")
        }
    }
}
```

---

## Exercícios

Fazer a tabela verdade do condicional
Fazer a tabela verdade do bicondicional
Fazer a tabela verdade do ou excluiso


---

## Vetor Array

```rust
let alfabeto: [char; 3] = ['a','b','c'];
```

[▶ executar](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=91d00c7a8e2654d4a0e234ad9dc7c724)

## Iteração

```rust
let naipes = ['🔶', '♠️', '🧡', '♣️'];
let fig = ['👸', '👨', '🤴', ];

for n in naipes {
    for f in fig {
        println!("{}{}", n, f);
    }
}
```

---

♠ Alt + 6
♣ Alt + 5
♦ Alt + 4
♥ Alt + 3

---

## Exercício 

Imprima a tabuada do 7;

---

## Exercício

Imprima a tabuada do 3 ao 9 multiplicando números de 1 a 10

---

## Exercício

Imprima todas as cartas do baralho

--- 

## Exercício Senha

Usando números de "0" a "9" escreva todas as senhas possívels com 2 caracteres

```rust
let alfa = "0".."9";
for a in alfa { 
    for b in alfa {

    }
    
}
```

--- 

## Exercício Senha

Usando números de "0" a "9" escreva todas as senhas possívels com 2 caracteres sem repeticao

```rust
let alfa = "0".."9";
for a in alfa {   
    for b in alfa {
        if a == b {
            continue
        }
        else {
            println!("{}{}", a, b)
        }

    }
    
}
```

## Exercício Senha

Usando letras de "A" a "M" escreva todas as senhas possívels com 3 caracteres sem repeticao

## Exercício Senha

Crie o sorteio do grupo G da Copa do mundo de 2022 sabendo que as seleções são Brasil Suíça Camarões Sérvia
Use os símbolos BRA SUI CAM SER

---

## Exercício Senha

Crie o sorteio do grupo G da Copa do mundo de 2022 sabendo que as seleções são Brasil Suíça Camarões Sérvia
Use os símbolos BRA SUI CAM SER

```rust
let grupo_g = ["BRA", "SUI", "CAM", "SER"];
for p in grupo_g {
    for q in grupo_g {
        println!("{}⨉{}", p, q)

    }
}
```

---

## Exercício Seleção

Imprima uma tabela com a classificação final sabendo que cada seleção alcançou x vitórias e z empates
Calcule e imprima o numero de jogos, o numero de pontos

```
	V	E
BRA	2	0
SUI	2	0
CAM	1	1
SER	0	1
```

---

```rust
let grupo_g = ["BRA", "SUI", "CAM", "SER"];
for p in grupo_g {
    for q in grupo_g {
        println!("{}⨉{}", p, q)

    }
}
```

---

## Produto cartesiano

$$Sejam os conjuntos M, N calcule o produto cartesiano M⨉N$$

```rust

//crie os elementos do conjunto
let R = "R";
let X = "X";
let A = "A";
let E = "E";
let I = "I";
let I = "O";
let U = "U";


//crie o conjunto
let M = [R, X];
let N = [A,E,I,O,U];

for m in M {
    for n in N {
        println!("{}{}", m, n);
    }
}
```

---

## Produto cartesiano

$$Sejam os elementos A..=Z, e os numeros 1..65535 desenhe todas as celulas possíveis da tabela do excel$$

```rust
print!("   |");
for x in 'A'..='Z' {
    print!(" {} |", x);
}
println!("");

for y in 1..16 {
    print!("{:2} |   ", y);
    for x in 'A'..'X' {
            print!("|   ");
        }
        println!("");
    }
    
    

```

## Funcoes

Crie um programa em rust que desenhe um grafico com os valores (x,y) informados em uma lista

--- 
## Tuplas

$$Sejam os conjuntos M, N encontre os pares ordenados M⨉N$$

```rust


//crie o conjunto
let M = "A"..="F";
let N = 0..=9;

for m in M {
    for n in 0..=3 {
        let po = (m, n);
        println!("{:?}", po);
    }

}