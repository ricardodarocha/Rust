/// # Programa CCaixa - Controle de Caixa
/// Um livro de caixa é uma reunião de lançamentos de entradas (recebimentos) 
/// e saídas (pagamentos), e possui operações de Lancamento, Soma das entradas,
/// Soma das saídas e Apuração do saldo
/// ▶ execute este programa https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b80ad83ad5560cb4d0c8df0deadd0f1e
struct Caixa {
    reg: Vec<Lancamento>,
}

struct Lancamento {
    val: f32,
    descr: String,
}

impl Caixa {

    /// Função para abrir o caixa, informando o `Saldo Inicial`
    fn abrir(saldo_inicial: f32) -> Self {
        let mut novo_caixa = Caixa {
            reg: vec![],
        };
        
        novo_caixa.registrar(saldo_inicial, "Abertura do caixa".to_string());
        novo_caixa
    }
    
    fn registrar_str(&mut self, val: f32, descr: &'static str) -> &mut Self {
        let reg: Lancamento = Lancamento{val, descr: descr.to_string()};
        self.reg.push(reg);
        self
    }
    
    fn registrar(&mut self, val: f32, descr: String) -> &mut Self {
        let reg: Lancamento = Lancamento{val, descr: descr};
        self.reg.push(reg);
        self
    }
    
    fn apurar(&self) -> (f32, f32, f32) {
        (self.entradas(), self.saidas(), self.saldo())
    }
    
    fn entradas(&self) -> f32 {
        self.reg
        .iter()
        .filter(|x|  x.val > 0.)
        .fold(0.0, |a, b| a + b.val)       
    }
    
    fn saidas(&self) -> f32 {
        self.reg
        .iter()
        .filter(|x|  x.val < 0.)
        .fold(0.0, |a, b| a - b.val) 
    }
    
    fn saldo(&self) -> f32 {
        self.entradas() - self.saidas()   
    }
}

fn imprimir_saldo(entrada: f32, saida: f32, saldo: f32) {
     print!("\n  {:>9.2} Entradas\n  {:>9.2} Saídas\n--------------------\n  {:>9.2} Saldo\n\n", entrada, saida, saldo);
     println!("─────────────────────────────────────────────────────────────────");
}

fn main() {
    //---- P A R T E 1 (B Á S I C O) ----------
    //---- código estático
    
    println!("Exemplo 1");
    
    let mut caixa = Caixa::abrir(500.0);
    caixa
        .registrar_str(-300., &"🏋️")
        .registrar_str(-120., &"📚")
        .registrar_str(3000., &"💰")
        .registrar_str(-300., &"💳")
        .registrar_str(-300., &"🍣")
        .registrar_str( 300., &"💹");
        
    for reg in &caixa.reg {
        println!("  {:>9.2} {}", reg.val, reg.descr);
    }
    
    let (e, s, c) = caixa.apurar();
    imprimir_saldo(e, s, c);
     
    //---- P A R T E 2 (A V A N Ç A D O) ----------
    //---- código interativo
    
    println!("Exemplo 2");
    
    println!("Vamos começar a interação\n Informe negativo para saídas e positivo para entradas");
    let saldo_inicial: f32;
    leia!(saldo_inicial: f32);
    println!("Abrindo o caixa com R$ {saldo_inicial:.2}");
    let mut caixa = Caixa::abrir(saldo_inicial);
    loop {
        let valor: f32;
        let descricao: String;
        
        leia!(valor: f32);
        if valor == 0.0 {
            let (total_entradas, total_saidas, saldo) = caixa.apurar();
            imprimir_saldo(total_entradas, total_saidas, saldo);
            break;
        } else if valor > 0.0 {
            println!("Qual a descrição da entrada?");
        } else if valor < 0.0 {
            println!("Qual a descrição da saída?");
        }
        leia!(descricao: String);
        println!("-------------------------------------------");
        println!("💲registrando R$ {valor:10.2} {descricao:9}");
        println!("-------------------------------------------");
        caixa.registrar(valor, descricao);
    }
}

pub mod portugol {
    #[macro_export]
    macro_rules! leia {
        ($var:ident : $t:ty) => {
            {
                use std::io::{self, Write};
                let mut input = String::new();
                println!("  informe o campo {}: ", stringify!($var));
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
                input = input.trim().to_string();
                print!("> {}\n", input);
                $var = input.parse::<$t>().expect("Falha na conversão do valor");
            }
        };
    }
}     
