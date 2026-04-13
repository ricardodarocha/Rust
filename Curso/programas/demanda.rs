///Programa para calcular demanda de itens

#![allow(dead_code)]
#![allow(unused)]

#[allow(non_camel_case_types)]
type int = i32;

#[allow(non_camel_case_types)]
type float = f64;

#[derive(Clone)]
struct ItemRastreavel {
    codbarras: String,
    nome: String,
    subtipo: String,
    estoque: int,
}

impl Default for ItemRastreavel {
    fn default() -> Self {
        Self { 
            codbarras: "".to_string(),
            nome: "".to_string(),
            subtipo: "".to_string(),
            estoque: 9990, 
        }
    }
} 

impl Display for ItemRastreavel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 

            write!(
                f,
                "{:<14} {:>5} ",

                self.nome,
                self.subtipo,
            )
    }
}

#[derive(Clone)]
struct ProdutoWrapper {
    produto: ItemRastreavel,
    preco: float,
    componentes: Vec<ItemComposicao>,
}

impl Display for ProdutoWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 

            write!(
                f,
                "{:>20} ",
                self.produto,
            )
    }
}

impl ProdutoWrapper {
    fn novo(nome: &str, preco: float, ) -> Self {
        Self{
            produto: ItemRastreavel {
                nome: nome.to_string(),
                ..ItemRastreavel::default() 
            },
            preco,
            componentes: vec![],
        }
    }
    
    fn add_component(&mut self, quantidade: int, componente: &ProdutoEnum) -> &mut Self {
        let unidade =  "UND".to_string();
        self.componentes.push(
            ItemComposicao {
                quantidade,
                unidade,
                componente: componente.clone(),
            }
        );
        self
    } 
}

#[derive(Clone)]
struct ComponenteWrapper {
    componente: ItemRastreavel,
}

impl ComponenteWrapper {
    fn novo(nome: &str, subtipo: &str, ) -> Self {
        Self{
            componente: ItemRastreavel {
                nome: nome.to_string(),
                subtipo: subtipo.to_string(),
                ..ItemRastreavel::default() 
            }, 
        }
    }
}

#[derive(Clone)]
struct MateriaPrimaWrapper {
    materia_prima: ItemRastreavel,
}

impl MateriaPrimaWrapper {
    fn novo(nome: &str, ) -> Self {
        Self{
            materia_prima: ItemRastreavel {
                nome: nome.to_string(),
                ..ItemRastreavel::default() 
            }, 
        }
    }
}

#[derive(Clone)]
enum ProdutoEnum {
    Produto(ProdutoWrapper),
    Componente(ComponenteWrapper),
    MateriaPrima(MateriaPrimaWrapper),
}

impl ProdutoEnum {
    fn nome(&self) -> String {
        match self {
            ProdutoEnum::Produto(p) => format!("{} {}", p.produto.nome, p.produto.subtipo),
            ProdutoEnum::Componente(c) => format!("{} {}", c.componente.nome, c.componente.subtipo),
            ProdutoEnum::MateriaPrima(m) => format!("{} ", m.materia_prima.nome ),
        }
    }
}

#[derive(Clone)]
struct ItemPedido {
    quantidade: int,
    unidade: String,
    item: ProdutoWrapper,
    total: float,
}

impl Display for ItemPedido {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 

            write!(
                f,
                "{:>20} {:>5} {:>10.2} {:>10.2}",
                self.item,
                self.quantidade,
                self.item.preco,
                self.total
            )
    }
}

#[derive(Clone)]
struct ItemComposicao {
    quantidade: int,
    unidade: String,
    componente: ProdutoEnum, 
}

impl Display for ItemComposicao {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 

          write!(
                f,
                "{:<15} {:>5}  ",
                self.componente.nome(),
                self.quantidade 
            )
    }
}

#[derive(Clone)]
struct Pedido {
    total: float,
    itens: Vec<ItemPedido>,
}

impl Pedido {
    fn novo() -> Self {    
        Self {
            total: 0.0,
            itens: vec![],
        }
        
    }
    
    fn add(&mut self, quantidade: int, item: &ProdutoWrapper) -> &mut Self {
        let unidade = "UND".to_string();
        let total = quantidade as float * item.preco;
        self.itens.push(
            ItemPedido{quantidade, unidade, total, item: item.clone()
            });
        self.total += quantidade as float * item.preco;
        self
    }
    
    //fecha o pedido e imprime a notinha
    fn totalizar(&mut self) -> &mut Self {   
        println!("{}", self);
        self 
    }
}

struct Lote {
    pedidos: Vec<Pedido>,
    componentes: Vec<ItemComposicao>,
}

impl Lote {
    fn novo() -> Self {
        Self {
            pedidos: vec![],
            componentes: vec![],
        }
    }
    
    fn add(&mut self, pedido: &Pedido) -> &mut Self { 
        self.pedidos.push(pedido.clone());

        for item_pedido in &pedido.itens {
            // para cada componente do produto
                // println!("{:#}", item_pedido);
            for comp in &item_pedido.item.componentes {
                // println!("       {:#}", comp);
                let quantidade_total = comp.quantidade * item_pedido.quantidade;
                
    
                match self.componentes
                    .iter_mut()
                    .find(|c| c.componente.nome() == comp.componente.nome() )
                {
                    Some(existing) => {
                        existing.quantidade += quantidade_total;
                    }
                    None => {
                        self.componentes.push(ItemComposicao {
                            quantidade: quantidade_total,
                            unidade: comp.unidade.clone(),
                            componente: comp.componente.clone(),
                        });
                    }
                }
            }
        }
    
        self
        
    }
} 

use std::fmt::Display;
use std::fmt::Formatter;

impl Display for Pedido {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {

        // Cabeçalho
        writeln!(f, "{:<20} {:>5} {:>10} {:>10}", "Item", "Qtd", "Preço", "Total")?;
        writeln!(f, "{}", "-".repeat(50))?;

        for item in &self.itens {
            writeln!(f, "{}", item)?;
        }

        writeln!(f, "{}", "-".repeat(50))?;
        
        
        // Total geral alinhado à direita
        writeln!(
            f,
            "{:>37} {:>10.2}",
            "TOTAL:",
            self.total
        )?;

        Ok(())
    }
}

fn main(){
    let pe_cadeira = ComponenteWrapper::novo("Pe ", "cadeira");
    let pe_mesa = ComponenteWrapper::novo("Pe", "mesa");
    let encosto = ComponenteWrapper::novo("Encosto", "");
    let assento = ComponenteWrapper::novo("Assento", "");
    let tecido = MateriaPrimaWrapper::novo("Tecido" );

    let mut cadeira = ProdutoWrapper::novo("Cadeira", 270.0);
    cadeira
        .add_component(4, &ProdutoEnum::Componente(pe_cadeira.clone()) )
        .add_component(2, &ProdutoEnum::Componente(encosto.clone()) )
        .add_component(1, &ProdutoEnum::Componente(assento.clone()) )
        .add_component(160, &ProdutoEnum::MateriaPrima(tecido.clone()) )
    ;
    
    let mut mesa = ProdutoWrapper::novo("Mesa", 800.0);
    mesa
        .add_component(4, &ProdutoEnum::Componente(pe_mesa.clone()) )
    ;

    let mut pedido = Pedido::novo();
    
    pedido
        .add(4, &cadeira)
        .add(1, &cadeira)
        .add(1, &mesa) 
        .totalizar();
        
    let mut pedidos = Lote::novo();
    pedidos.add(&pedido);
        
    for componente in pedidos.componentes {
        println!("{}", componente);
    }
    //pedido.aprovar(); //Financeiro aprova o pedido
    //pedido.produzir(); //Incluir na lista de fabricação
    //pedido.embalar(); //Expedição embala a mercadoria
    //pedido.faturar(); //gera nota fiscal e faturas de cobrança
    //pedido.carregar(); //Expedição coloca na transportadora

}

//https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=05f0440d7ba3137a7564043ca2d84962
