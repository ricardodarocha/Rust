#![allow(dead_code)]
#![allow(unused)]
macro_rules! imprima {
    ($($arg:tt)*) => {{
        let formatted = format!("{}", format_args!($($arg)*));
        let formatted = formatted
            .replace("true", "V")
            .replace("false", "F");
        println!("{}", formatted);
    }};
}
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
type Texto = String;

fn main {
  let mut numero: Inteiro;
  imprima!("Informe um número", );  
  leia!(numero: Inteiro);

  imprima!("Tabuada do {numero} ", ); 
  imprima!("====================", );
  for i in 0..10 {
          imprima!("{numero} * {i} = {}" , numero * i); // Imprime numero * i
      }
  
}
