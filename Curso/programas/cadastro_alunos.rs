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

/// Este programa possui a struct Aluno:
/// nome (String),
/// matrícula (i32),
/// nota1 e nota2 (f32).
/// O programa deve:
/// Ter um menu para o usuário:
/// (1) Cadastrar aluno
/// (2) Listar alunos e suas médias
/// (3) Sair
/// Usar funções para:
/// cadastrar um aluno,
/// calcular a média,
/// imprimir os dados.
pub mod aluno {

    pub struct Aluno{
        pub matricula: i32, 
        pub nome: String, 
        pub nota1: f32, 
        pub nota2: f32,
    }

    pub fn novo() -> Aluno {
    
    // let (matricula, nome, nota1, nota2):(i32, String, f32, f32);
    // leia!(matricula: i32);
    // leia!(nome: String);
    // leia!(nota1: f32);
    // leia!(nota2: f32);
      
      Aluno {
          matricula: 1,
          nome: "Fulano".to_string(),
          nota1: 9.0,
          nota2: 8.5
      }
      
    }
    
    
    pub fn listar(alunos: &Vec<Aluno>) {
      for aluno in alunos {
          println!("{} {}", aluno.nome, (aluno.nota1 + aluno.nota2) / 2.0);
      }  
    }
}

pub mod app {
    use crate::leia;
    
    pub enum Acao {
        Cadastrar,
        Listar,
        Sair,
    }

    pub fn menu() -> Acao {
        let opcao: i8;
        println!("
            (1) Cadastrar
            (2) Listar
            (3) Sair
        
        ");
        leia!(opcao: i8);
        
        match opcao {
            1 => Acao::Cadastrar,
            2 => Acao::Listar,
            3 => Acao::Sair,
            _ => menu(),
        }
    }
}

fn main() {
    let mut alunos: Vec<aluno::Aluno> = vec![];
    use app::Acao;
    
    loop {
        let acao = app::menu();
        match acao {
            Acao::Cadastrar => alunos.push(aluno::novo()),
            Acao::Listar => aluno::listar(&alunos),
            Acao::Sair => break,
        }
    }
}
