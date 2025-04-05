#![allow(dead_code)]
macro_rules! leia {
    ($var:ident : $t:ty) => {
        {
            use std::io::{self, Write};
            let mut input = String::new();
            println!("informe {}: ", stringify!($var));
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
            input = input.trim().to_string();
            print!("> {}\n", input);
            $var = input.parse::<$t>().expect("Falha na conversão do valor");
        }
    };
}
type Real = f64;

fn main() {

  loop {
    let num1: Real;
    let mut maior = num1;
    let mut menor = num1;
    
    let mut numero: Real;
    
    if numero < 0 {
      break
    } else
    {
      if numero > maior {
        maior = numero;
      }  
      if numero < menor {
        menor = numero;
      }  
    }
    
    println!("Para interromper, informe < 0");
    continue;
  }
  println!(">{maior}, <{menor});
  
}
