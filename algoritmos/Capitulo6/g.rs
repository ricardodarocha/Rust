fn main() {
  let mut e = 0;
  let base = 3;
  let mut x: i32;
  
  while e <=15 {
    if e == 0 {
      x = 1;
    } 
    
    if e == 1 {
      x = base;
    }  else {
      x = x * base; // x *= base
    }

    print!("{}, ", x)
      
    i+=1;
  } 
  
}
