#[derive(Debug)]
struct MinhaArvore {
    nome: String,
    ramos: Option<Box<MinhaArvore>>,
}

fn main() {
    let galileu = MinhaArvore {
        nome: "Galileu",
        ramos: Some(Box::new(MinhaArvore{
            nome: "Newton",
            ramos: Some(Box::new(MinhaArvore){
                nome: "Einstein",
                ramos: Some(Box::new(MinhaArvore))
                })
            })
        )
    }
}

println!("{#?}", galileu);

