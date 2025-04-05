fn main() {
  let mut i = 1;
  let mut q = 1;
  let mut soma = 0;
    
  loop {
    soma += q;
    i += 1;
    q *= 2;
    if i > 64 {
      break;
    } 
  } 
  println!("{soma}");
  
}
