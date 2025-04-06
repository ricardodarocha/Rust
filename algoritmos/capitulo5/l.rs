#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused)]

macro_rules! leia {
    ($var:ident : $t:ty) => {
        {
            use std::io::{self, Write};
            let mut input = String::new();
            // println!("o sistema está aguardando digitar um valor para {}: ", stringify!($var));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
            input = input.trim().to_string();
            print!("> {}\n", input);
            $var = input.parse::<$t>().expect("Falha na conversão do valor");
        }
    };
}
type Inteiro = i64;
type int = i64;

fn main() {
  let mut contador = 1;
  let mut soma = 0;
  let max = 9;
  
  loop {
    print!("\nInforme um número [{n}/{m}]", n = contador, m = max);
    let mut numero: int;
    leia!(numero: int);
    
    let mut fatorial = numero;

    while numero > 1 {
      print!("{}, ", fatorial); //🔥
      numero = numero - 1; 
      fatorial = numero * fatorial;
    }
      
    soma = soma + fatorial;
    print!("= {} \n", fatorial); //🔥
    
    contador = contador + 1;
    if contador > max {
      break;
    } 
  } 
  println!("\n---------------------");
  println!("SOMA == {}", soma);
  println!("---------------------");
  println!("fim do programa")
  
}
