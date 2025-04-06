fn main() {
  type int = i32;
  let mut soma: int;
  let mut quant: int;

  for i in 50..=70 {
    if i % 2 == 0 {
      soma += i;   
      quant += 1;
    }
  }

  let media = soma / quant;
  println!("{}, {}", soma, media);
  
}
