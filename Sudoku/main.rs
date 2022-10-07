fn main() {

    let mut sudo = [[0 as u8; 9]; 9];
    sudo[2][3] = 1;
    sudo[2][4] = 5;
    
    imprimir(sudo);
   
}

fn imprimir(sudo : [[u8; 9]; 9]) {
    for y in sudo {
        print!("   ");
        for x in y {
            print!("{:?} ", x);
        }
        println!();
    }
}

fn indice(x: u8, y: u8) -> u8 {
    9*x+y
}

fn linha(indice: u8) -> u8 {
    (indice-1) / 9 
}

fn coluna(indice: u8) -> u8 {
    (indice-1) % 9 
}
