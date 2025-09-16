//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bea6a0c99127539b00256acfbda45305
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
}
 
const E0: f64 = 7e-3;
fn magnitude_from_amplitude(a: f64, delta_t: f64) -> f64 {
    a.log10() + 3.0 * (8.0 * delta_t).log10() - 2.92
}

fn magnitude_from_energy(e: f64) -> f64 {
    (2.0 / 3.0) * (e / E0).log10()
}

fn energy_from_magnitude(m: f64) -> f64 {
    const E0: f64 = 7e-3;
    E0 * 10f64.powf(1.5 * m)
}

fn exibir_alerta(m: f64) {
    let limites = [8.0, 7.0, 6.1, 5.5, 3.5, 0.0];
    let mensagens = [
        "🚨 ALERTA: TERREMOTO ENORME (Danos severos a longas distâncias)",
        "🔴 ALERTA: GRANDE TERREMOTO (Terremoto altamente destrutivo detectado)",
        "🔶 ALERTA: TERREMOTO  (Destrutivo em até 100 km do epicentro)",
        "🟠 ALERTA: TERREMOTO  (Danos prováveis)",
        "🟡 Terremoto Registrado (Sem danos)  ",
        "🟢 Registrado (não sentido) ",
    ];

    for (i, &limite) in limites.iter().enumerate() {
        if m >= limite {
            println!("Magnitude {:.3} → {}", m, mensagens[i]);
            break;
        }
    }
}

fn _menu() -> i8 {
    println!("(1) Magnitude da amplitude (mm)");
    println!("(2) Magnitude da energia despendida E");
    let opcao: i8;
    leia!(opcao: i8, "\nModo: ");
    opcao
}

fn main() {

    loop{

        // let opcao = menu();
        let opcao = 1;
        
        if opcao == 1 {
            let a: f64;
            let delta_t: f64;
            leia!(a: f64, "Informe a Amplitude A (mm)");
            leia!(delta_t: f64, "Informe ΔT (s)");

            let m = magnitude_from_amplitude(a, delta_t);
            exibir_alerta(m);

            let e = energy_from_magnitude(m);
            println!("Energia despendida (E) = {:.2E}", e);
        } else if opcao == 2 {
            let e: f64;
            println!("");
            leia!(e: f64, "Informe a Energia despendida(E)");
            
            let m = magnitude_from_energy(e);
            println!("Magnitude (E) = {:.2} kWh", m);
        } else if opcao == 3 {
            break;
        } else {
            continue;
        }
    }
}
