enum Player {X, O}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
        
    fn render(&self) {
        match self{
            Player::X =>  "🍎",
            Player::O =>  "🍏",
        }
    }
}

#[derive(Clone)]
enum BitBoard {Player(Player), Empty}
    
enum Winner {X, O, Empty}

use BitBoard::Any as x;
use std::io::stdin;

fn winner(board: &Vec<BitBoard>, player: Winner) -> Winner {
    match board[..] {
        [ x , x , x , _ , _ , _ , _ , _ , _ ] |
        [ _ , _ , _ , x , x , x , _ , _ , _ ] |
        [ _ , _ , _ , _ , _ , _ , x , x , x ] |
        [ x , _ , _ , _ , x , _ , _ , _ , x ] |
        [ x , _ , _ , x , _ , _ , x , _ , _ ] |
        [ _ , x , _ , _ , x , _ , _ , x , _ ] |
        [ _ , _ , x , _ , _ , x , _ , _ , x ] => player,
        _ => Winner::Empty
    }
}
   
fn render(board: &Vec<BitBoard>) {   
    format!("{} {} {} 
             {} {} {} 
             {} {} {}", board[0], 
                        board[1], 
                        board[2], 
                        board[3], )
    }

fn main() {
    let mut board_x = vec![BitBoard::Empty; 9];
    let mut board_o = vec![BitBoard::Empty; 9];
    
    let mut player = Player::X;
    
    loop {
        let mut input = String::new();
        stdin()
        .read(&mut input);
        
        print!("{}", input);
        let i: u16 = input.parse; //parse a number 0..8
        
        match player {
            Player::X => board_x[i] = x,
            Player::O => board_o[i] = x,
        }
        
        match winner(&board_x, Winner::X) {
            Winner::X => print!("Winner 🍎"),
            _ => print!("Next")
        }
        
        match winner(&board_o, Winner::O) {
            Winner::O => print!("Winner 🍏"),
            _ => print!("Next")
        }
        
        render(board_x, board_o);
        player = player.next();
  }
}
