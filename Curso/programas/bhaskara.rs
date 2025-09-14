pub mod portugol {
    #[macro_export]
    macro_rules! leia {
        ($var:ident : $t:ty) => {
            {
                use std::io::{self, Write};
                let mut input = String::new();
                println!("  informe o campo {}:{} ", stringify!($var), stringify!($t), );
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
                input = input.trim().to_string();
                print!("> {}\n", input);
                $var = input.parse::<$t>().expect("Falha na conversão do valor");
            }
        };
    }
}     

pub mod bhaskara {

    pub type Num = f32;

    #[derive(Debug)]
    pub enum BResult {
        //f32 = (+-)0..2^5
        DuasRaizes(Num, Num),
        UmaRaiz(Num),
        Erro(String)
    }
    
    
    /// (X1, X2)
    /// X
    /// Erro -> a=0 (Nao e funcao do segundo grau)
    /// Erro -> delta=<0 (Nao existe raiz real)
    pub fn calc(a: Num, b: Num, c: Num) -> BResult {
      
      if a==0.0 {
          return BResult::Erro("Não é uma equação do segundo grau".to_string());
      }
      
      let delta = b*b-4.0*a*c;
      if delta < 0.0 {
          return BResult::Erro("Nao possui raizes reais".to_string());
      }
      
      if delta == 0.0 {
          return BResult::UmaRaiz(-b / 2.0*a);
      }
      
      let x1 = -b + f32::sqrt(delta)/ 2.0*a;
      let x2 = -b - f32::sqrt(delta)/ 2.0*a;
      
      
      return BResult::DuasRaizes(x1, x2);
    }
}

fn main() {
    let (a, b, c):(f32, f32, f32);
    leia!(a: f32);
    leia!(b: f32);
    leia!(c: f32);
    let result = bhaskara::calc(a, b, c);
    println!("{:?}", result);
    
}
