# Matrizes

Há duas formas de trabalhar com matrizes no Rust,  o que deverá ser levado em conta é se a matriz pussui dados constantes ou mutáveis, e se o tamamho da matriz é constante ou mutável.

Uma matriz de tamanho constante pode ser representado como um array  

## Array 

```Rust
let fib9 = [1,1,2,3,5,8,11,19,30]
//o mesmo
let fib9: [u8;9] = [1,1,2,3,5,8,11,19,30];
```

Note que esta array não pode ser alterada.
E também não podem ser inseridos novos valores nesta array.

## Arrays mutáveis

Para trabalhar com um array mutável adiciona a palavra mut

```Rust
let mut fib9: [u8;9] = [1,1,2,3,5,8,11,19,30];
fib9[0]=0
```

Agora podemos alterar um elemento deste array, contanto que o elemento esteja no intervalo \[0..9\]  
No entanto ainda não é possível inserir um novo elemento no array; Isto porque o array é um tipo de tamanho constante em Rust.

## Matrizes dinâmicas, Vetores

Para trabalhar com matrizes dinâmicas é necessário declarar o tipo vetor

```Rust
let mut fib9: Vec<u8> = vec![1,1,2,3,5,8,11,19,30];
fib9[0]=0;
print!("{:?}", fib9)
```

Para inserir novos elementos é necessário chamar o método **push()**

```Rust
let mut fib9: Vec<u8> = vec![1,1,2,3,5,8,11,19,30];
fib9.push(49);
print!("{:?}", fib9)
```

# Matrizes bidimensionais

```Rust
 let mut vec = vec![vec!['⬜'; 3]; 3];
 fn renderizar(board:&Vec<Vec<char>>) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{}", board[i][j])
        }
        print!("\n")
    }
    print!("\n")
    
}

```
