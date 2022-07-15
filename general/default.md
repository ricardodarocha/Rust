
# Valores Default

Imagine o seguinte conjunto de emojis representando um jogo da velha.
Queremos iniciar nosso jogo da velha sempre com o valor "vazio"

Nós podemos criar um enumerado com a diretiva `derive(Default)` ou implementar o trait Default. As duas formas são equivalentes  

⭕❌⬜

### Forma 1  

```Rust
#[derive(Default)]
enum Tab{
        O,
        X,
        #[default]
        Vazio
    }
```
    
### Forma 2  

```Rust
enum Piece{O, X, Vazio}
impl Default for Tab -> Self {Piece::Vazio}
```

o trait Default também pode ser implementado para objetos

```
struct Jogador {
nome: String,
idade: u16}

impl Default for Jogador -> Self {
  Jogador {
    nome: "Anônimo".to_own,
    idade: 0
  }
}
```
