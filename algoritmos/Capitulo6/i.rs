fn main() {
    let mut anterior = 0; //experimente comentar esta linha
    let mut atual = 0;
    let mut proximo = 1;
    let mut contador = 1;
    
    loop{
        println!("{:>2} {} ", contador, atual);
        
        anterior = proximo;
        proximo = atual;
        atual = anterior + proximo;
        contador+=1;
    }
}

/// warning: unused variable: `anterior`
/// let mut anterior = 0;
///  |             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_anterior`
///  |
///  = note: `#[warn(unused_variables)]` on by default
