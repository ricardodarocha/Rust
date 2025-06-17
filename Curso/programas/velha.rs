#![allow(dead_code)]

//▶ https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dd40c034d23fa228094f0ddc1367d58b

use std::ops::{Index, IndexMut};

struct Board<T, const SIZE: usize> {
    cols: usize,
    tiles: [T; SIZE],
}

// impl<T, const SIZE: usize> Board<T, SIZE> {
impl<T: Copy + Default, const SIZE: usize> Board<T, SIZE> {
    fn new(cols: usize) -> Self {
        Self {
            cols,
            tiles: [T::default(); SIZE],
        }
    }

    fn value(&self, x: usize, y: usize) -> T {
        self.tiles[self.index(x, y)]
    }

    fn chunk(&self, index: usize) -> &[T] {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.tiles[start..end]
    }

    fn vchunk(&self, col: usize) -> impl Iterator<Item = &T> {
        let width = self.lines() as usize; // assuming lines() == width
        (0..width).map(move |row| &self.tiles[row * width + col])
    }

    fn set_value(&mut self, index: usize, value: T) {
        self.tiles[index] = value;
    }

    fn set_valuexy(&mut self, x: usize, y: usize, value: T) {
        self.tiles[self.index(x, y)] = value;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.cols
    }

    fn col(&self, x: usize) -> u16 {
        (x % self.cols) as u16
    }

    fn lin(&self, x: usize) -> u16 {
        (x / self.cols) as u16
    }

    fn lines(&self) -> u16 {
        ((SIZE - 1) / self.cols + 1) as u16
    }

    fn same_col(&self, a: usize, b: usize) -> bool {
        self.col(a) == self.col(b)
    }

    fn same_lin(&self, a: usize, b: usize) -> bool {
        self.lin(a) == self.lin(b)
    }

    fn same_diagonal(&self, a: usize, b: usize) -> bool {
        let (x0, y0) = (self.col(a) as i16, self.lin(a) as i16);
        let (x1, y1) = (self.col(b) as i16, self.lin(b) as i16);
        (x1 - x0).abs() == (y1 - y0).abs()
    }
}

impl<T, const N: usize> Index<usize> for Board<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Board<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

struct Velha {
    board: Board<i8, 9>,
    n: u8,
}

use std::fmt::{Display, Formatter, Result};

impl Display for Velha {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..9 {
            let repr = match self.board[i] {
                x if x == Self::X => '❌',
                o if o == Self::O => '⭕',
                _ => '⬜',
            };
            write!(f, "{}", repr)?;

            // Print a line break after every 3 tiles (for a 3x3 board)
            if (i + 1) % 3 == 0 {
                writeln!(f)?;
            } else {
                write!(f, "")?; // spacing between tiles
            }
        }
        Ok(())
    }
}

impl Velha {
    pub const X: i8 = 1;
    pub const O: i8 = -1;
    pub const EMPTY: i8 = 0;

    fn x(&mut self, index: usize) -> &mut Self {
        self.board[index] = Self::X;
        self.n += 1;
        self
    }

    fn o(&mut self, index: usize) -> &mut Self {
        self.board[index] = Self::O;
        self.n += 1;
        self
    }
    fn clear(&mut self, index: usize) -> &mut Self {
        self.board[index] = Self::EMPTY;
        self.n -= 1;
        self
    }

    fn winner(&self, player: i8) -> bool {
        if self.n < 3 {
            return false;
        }

        for i in 0..3 {
            if self.board.chunk(i).iter().all(|&v| v == player) {
                return true;
            }
        }

        for i in 0..3 {
            if self.board.vchunk(i).all(|&v| v == player) {
                return true;
            }
        }

        if (0..=8)
            .step_by(4)
            // .collect::<Vec<_>>()
            .all(|i| self.board[i] == player)
        {
            return true;
        } else if (2..=6)
            .step_by(2)
            // .collect::<Vec<_>>()
            .all(|i| self.board[i] == player)
        {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    const SIZE: usize = 64;
    let board: Board<i8, SIZE> = Board::new(8);
    println!(
        "Board with {} columns {} lines and {} tiles ({},{})",
        board.cols,
        board.lines(),
        board.tiles.len(),
        board.col(SIZE - 1) + 1,
        board.lin(SIZE - 1) + 1
    );

    let mut velha: Velha = Velha {
        board: Board::new(3),
        n: 0,
    };

    // velha.x(3).o(2);
    // print!("{}", velha);
    use rand::Rng;
    for player in [Velha::X, Velha::O].into_iter().cycle() {
        println!("Player: {}", player);
        
        let mut rng = rand::rng();
        let mut r = rng.random_range(0..9);
        
        while velha.board[r] != Velha::EMPTY {
            r = rng.random_range(0..9);    
        }
        
        if player == Velha::X {
            velha.x(r);
        } else 
        if player == Velha::O {
            velha.o(r);
        }
        
        print!("{}", velha);

        if velha.winner(player) {
            println!("🏆 {}", player);
            break;
        }
        
        if velha.n == 9 {
            println!("🤝 {}", "Empate");
            break
        }   
    }
}
