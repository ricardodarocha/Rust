#![allow(dead_code)]
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

fn main() {

  loop {
    let mut nome: String;
    let mut comp: Inteiro;
    let mut larg: Inteiro;
    leia!(nome: String);
    leia!(comp: Inteiro);
    leia!(larg: Inteiro);

    println!("{nome}, {area}", area = comp * larg);
    // leia!(input: String);
    // if input == "NÃO" { break }

    println!("Para interromper, pressione enter");
  }
  
}
