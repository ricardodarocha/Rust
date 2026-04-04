pub mod portugol {
    #[macro_export]
    macro_rules! leia {
        ($var:ident : $t:ty, $msg:expr) => {
            {
                use std::io::{self, Write};
                let mut input = String::new();
                println!("{}", $msg);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
                input = input.trim().to_string();
                print!("> {}\n", input);
                $var = input.parse::<$t>().expect("Falha na conversão do valor");
            }
        };
        ($var:ident : $t:ty) => {
            leia!($var : $t, concat!("Informe o campo ", stringify!($var), ":", stringify!($t)));
        };
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
