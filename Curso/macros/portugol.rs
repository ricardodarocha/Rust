pub mod portugol {

    pub mod types {
        type Logico = bool;
        type Int = i32;
        type Num = f32;
        type Str = String;
        type Car = Char;
        type Byte = u8;
    
        type Inteiro = Int;
        type Numerico = Num;
        type Caractere = Car;
            
        Const SIM: Logico = true;
        Const NAO: Logico = false;
    
        Const S: Logico = SIM;
        Const N: Logico = NAO;
    }

    
    #[macro_export]
    macro_rules! leia {

        // Caso 1: sem label → delega para o caso 2
        ($var:ident : $t:ty) => {
            leia!(
                $var : $t,
                concat!(
                    "informe o campo ",
                    stringify!($var),
                    ":",
                    stringify!($t)
                )
            )
        };

        // Caso 2: implementação real
        ($var:ident : $t:ty, $label:expr) => {{
            use std::io::{self, Write};

            let mut input = String::new();

            println!("{}", $label);
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Falha ao ler entrada");

            let input = input.trim();

            $var = input
                .parse::<$t>()
                .expect("Falha na conversão do valor");
        }};
    }

  macro_rules! imprima {
      ($fmt:expr, $($arg:tt)*) => {
          println!(concat!("[APP] ", $fmt), $($arg)*);
      };
  
      ($msg:expr) => {
          println!(concat!("[APP] ", $msg));
      };
  }
}
