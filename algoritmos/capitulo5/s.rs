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
type int = i32;

fn main() {
  let mut q = 0;

  let mut a: int;
  leia!(a: int);
  
  let mut b: int;
  leia!(b: int);
  
  if b != 0 {
    if a > b {
      while q * b < a {
        q += 1;
      }
    }
    println!("{a}/{b}={q}");
  } else {
    println!("indeterminação";
  }  
}
