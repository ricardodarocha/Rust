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
  let mut soma = 0;
  let mut contador = 0;

  loop {
    let mut numero: int;
    leia!(numero: int);
    if numero >= 0 {
      soma += numero;
      continue;
    } else {
      break
    }
  }

  print!("{soma}, {media}, {contador}", soma, media = soma/quant, contador);
  
}
