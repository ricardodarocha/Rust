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
type Inteiro = i64;
type Texto = String;

fn main() {

  loop {
    let nome: String;
    let comp: Inteiro;
    let larg: Inteiro;
    leia!(nome: String);
    leia!(comp: Inteiro);
    leia!(larg: Inteiro);

    println!("área_{nome} = {area} m²", area = comp * larg);
    // leia!(input: String);
    // if input == "NÃO" { break }

    println!("\nPróximo\nPara interromper, pressione <enter> <enter>");
  }
  
}
