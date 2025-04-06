fn main() {
    let mut i = 15;
    
    // Usando while
    while i <= 200 {
        println!("{}", i * i); // Imprime i^2
        i += 1;
    } 
  
  // Usando loop
    loop {
        if i > 200 {
            break;
        }
        println!("{}", i * i); // Imprime i^2
        i += 1;
    }

   // Simples
   // Usando for sem intervalo implícito e sem iter
    for i in 15..=200 {
        println!("{}", i * i); // Imprime i^2
    }

  //avançado
  //Usando for com intervalo implícito (com iter() e map):  
  (15..=200).for_each(|i| println!("{}", i * i));
  
}
