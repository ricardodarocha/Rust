const CAMISAS: usize = 3;
const BOTOES: usize = 2;
const MESES: usize = 2;


/// Matriz pedidos[CAMISAS][MESES]

/// Camisa	Maio	Junho
/// 0	    100	    50
/// 1	    50	    100
/// 2	    50	    50
/// 🧮 Cálculo do Consumo

/// O consumo de cada botão em cada mês é dado por:

/// consumo[b][m] = Σ ( pedidos[c][m] * produtos[c][b] )

/// 📊 Resultado Final
/// Mês	Botão	Consumo
/// Maio	P    500
/// Maio	G	1150
/// Junho	P	 400
/// Junho	G	1150

fn main() {
    let nome_botao: [&str; BOTOES] = ["P", "G"];
    let nome_mes: [&str; MESES] = ["Maio", "Junho"];

    let produtos: [[i32; BOTOES]; CAMISAS] = [
        [3, 6],
        [1, 5],
        [3, 5],
    ];

    let pedidos: [[i32; MESES]; CAMISAS] = [
        [100, 50],
        [50, 100],
        [50, 50],
    ];

    let mut consumo: [[i32; MESES]; BOTOES] = [[0; MESES]; BOTOES];

    for m in 0..MESES {
        for b in 0..BOTOES {
            consumo[b][m] = 0;
            for c in 0..CAMISAS {
                consumo[b][m] += pedidos[c][m] * produtos[c][b];
            }
            println!("{:6} {:3} {:-6}.00", nome_mes[m], nome_botao[b], consumo[b][m]);
        }
    }
}
