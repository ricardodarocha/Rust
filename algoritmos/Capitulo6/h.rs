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
type int = i64;

fn main() {
  let mut base: int;
  let mut eleva: int;
  
  let mut x: i32;
  let mut e = 0;
  
  loop {
    if e == 0 {
      x = 1;
    } 
    
    if e == 1 {
      x = base;
    }  else {
      x = x * base; // x *= base
    }

    print!("{}, ", x)
      
    i+=1;

     if e > eleva break;
  } 

  print!("{x}");
  
}
