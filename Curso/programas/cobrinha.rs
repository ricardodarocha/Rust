/// [▶ execute este programa](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fa5763f6a7e6968c8eb9697980fc8dcb)

#![allow(dead_code)]

const L: usize = 17;
const SIZE: usize = L * L;

fn main() {

    //Este codigo utiliza um vetor unidimensional
    let mut board: [i32; SIZE] = [0; SIZE];
    
    //Para trabalhar com x, y transforme em uma matriz
    // let mut board2: [[i32; SIZE]; SIZE];
    //board2 [2][2] = 1;//
    //board2 [2][3] = 1;//
    //board2 [2][4] = 1;//
    //board2 [2][1] = 1;//
    //board2 [4][8] = -1;//
    
    //cria uma cobrinha ficticia
    board [36] = 1;
    board [37] = 2;
    board [38] = 3;
    board [39] = 4;
    board [40] = 5;
    
    //cria uma comida
    board [76] = -1;
    
    //direcao horizontal pra frente (1); pra tras (-1); parado (0);
    //direcao vertical pra baixo (1); pra cima (-1); parado (0);
    let mut direction: (i32, i32) = (1, 0);
    //      -1
    //       ^
    //-1 <-- 0 --> 1
    //       v 
    //       1
    
    loop {
      println!("      ⬆  \n      w  \n  <- asd ->\n      ⬇  " );
      draw(&board);
      let key: char;
      leia!(key: char);    
      match key {
          'w' => {
            direction = (0, -1);  
            next(&mut board, direction);
          },
          'a' => {
            direction = (-1, 0);  
            next(&mut board, direction);
          },
          's' => {
            direction = (0, 1);  
            next(&mut board, direction);
          },
          'd' => {
            direction = (1, 0);  
            next(&mut board, direction);
          },
          'x' => return ,
          _ => next(&mut board, direction),
      }
     }
}

fn next(board: &mut[i32; SIZE], direction: (i32, i32)) {
    const FOOD: i32 = -1;
    let mut cabeca = -1; //sq
    let mut n = -1; //id 1,2...n
    
    for i in 0..SIZE {
        if board[i] > n {
            n = board[i];
            cabeca = i as i32;
        }
    }
        
    let (x, y) = coord(cabeca);
    let (a, b) = direction;
    let mut next = (x + a, y + b);
    
    //Anda pra tras
    if (n>1) && (board[square(next)] == n-1) {
        let tone = n+1; 
        for i in 0..SIZE {
            if board[i] == 1 {
                //Era a cauda, agora vai virar a cabeca
                cabeca = i as i32;
                //Recalcula o next
                let (x, y) = coord(cabeca);
                next = (x + b, y + a);
            }
        }
            
        //atualiza a direcao 
        for i in 0..SIZE {
            if board[i] > 0 {
                board[i] = tone - board[i]; 
            }
        }
    }
    
    //shadowing nao permite mais alterar o next 🔐
    let next = next;
    
    if colide(next) {
        board[square(next)] = n;
        *board = game_over(board);
    } else if embaraca(&board, next) {
        board[square(next)] = n;
        *board = game_over(board);
    } else if board[square(next)] == FOOD {
        board[square(next)] = n + 1;
    } else {
        for i in 0..SIZE {
            if board[i] >= 1 {
                board[i] = board[i]-1; }
        }
        board[square(next)] = n;
    }
}

fn colide(destino: (i32, i32)) -> bool {
    let z = L as i32;
    let (a, b) = destino;
    a < 0 || 
    b < 0 || 
    a>= z ||
    b>= z 
}

fn embaraca(board: &[i32; SIZE], destino: (i32, i32)) -> bool {
    board[square(destino)] > 1
}

fn game_over(board: &[i32; SIZE]) -> [i32; SIZE] {

    const GAME_OVER: [usize; 82] = [
        34, 35, 36, 38, 39, 40, 42, 43, 45, 46, 48, 49, 50, 51, 55, 57,
        59, 61, 63, 65, 66, 67, 68, 70, 71, 72, 73, 74, 76, 78, 80, 82,
        85, 86, 87, 89, 91, 93, 97, 99, 100, 101, 119, 120, 121, 123, 127, 129,
        130, 131, 133, 134, 135, 136, 138, 140, 141, 143, 144, 146, 147, 148, 150, 151,
        152, 153, 155, 158, 159, 160, 163, 167, 168, 170, 171, 172, 176, 180, 181, 182,
        184, 186
    ];
    
    let mut result = [-2; SIZE];
    for indice in 0..SIZE {
        if board[indice] > 0 {
            result[indice] = board[indice] 
        }
    }
    for indice in GAME_OVER {
        result[indice + 34] = -3;
    }
    result
}

fn coord(sq: i32) -> (i32, i32) {
    let y = sq / L as i32;
    let x = sq % L as i32;
    (x, y)
}

fn square(coord: (i32, i32)) -> usize {
    let z = L as i32;
    let (a, b) = coord;
    (b * z + a) as usize
}

#[test] 
fn test_coord () {
    assert!(coord(0) == (0,0));
    assert!(coord(1) == (1,0));
    assert!(coord(15) == (15, 0));
    assert!(coord(16) == (0,1));
    assert!(coord(17) == (1,1));
}

#[test] 
fn test_square () {
    assert!(0 == square((0,0)));
    assert!(1 == square((0,1)));
    assert!(15 == square((0,15)));
    assert!(16 == square((1,0)));
    assert!(17 == square((1,1)));
}

fn fm(val: i32) -> char {
    match val {
        -3 => '🟨',
        -2 => '💣',
        -1 => '🍎',
        0 => '⬛' ,
        _ => '🟩',
        // x => char(x)
        
    }
}

fn char(n: i32) -> char {
    if n >= 0 && n <= 9 {
        // Convert to char by adding to '0' (ASCII math)
        (b'0' + n as u8) as char
    } else {
        '\0' // Null character (represents "nothing")
    }
}

fn draw(board: &[i32; SIZE]) {
    for row in board.chunks(L) {
        for sq in row {
            print!("{}", fm(*sq));
        }
        println!("");
    }
}

pub mod portugol {
    #[macro_export]
    macro_rules! leia {
        ($var:ident : $t:ty) => {
            {
                use std::io::{self, Write};
                let mut input = String::new();
                // println!("  informe o campo {}: ", stringify!($var));
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
                input = input.trim().to_string();
                print!("> {}\n", input);
                $var = input.parse::<$t>().expect("Falha na conversão do valor");
            }
        };
    }

}


