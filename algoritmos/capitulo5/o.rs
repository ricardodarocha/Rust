fn main() {  
  for contador in 1..=10 { //----------------------------+
    let mut numero = contador;                        // |
    let mut fatorial = numero;                        // |
                                                      // |
    while numero > 1 {                  //------+     // |
      print!("{}, ", fatorial); //🔥         // |     // | 
      numero = numero - 1;                   // |     // |
      fatorial = numero * fatorial;          // |     // | 
    }                                   //------+     // |
                                                      // |
    contador = contador + 2;                          // |
    println!("SOMA == {}", soma);                     // |
                                                      // |
  }                       //-----------------------------+
  println!("---------------------");
  println!("fim do programa")
  
}
