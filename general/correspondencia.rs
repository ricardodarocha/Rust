# O padrão Match

o padrão match é um dos recursos mais importantes da linguagem Rust

```Rust
let x: i32 = 1;

match x {
  1 => println!("um"),
  2 => println!("dois"),
  3 => println!("três"),
  - => println!("outro qualquer"),
}
```

```Rust
let x  = Some(5);
//let y = 10; //Isto jamais será usado

match x {
  Some(1) => println!("um"),
  1|2 => println!("um ou dois"),
  3..9 => println!("três .. nove"),
  Some(y) => println!("{}", y),
  _ => println!("qualqeur outro"),
}
```

```Rust
enum Color {
  Rgb(i32,i32,i32),
  Hsv (i32,i32,i32),
}

let color = Hsv(0, 160, 255);

match color {
  Collor::Rgb(r, g, b) => println!("RGB {} {} {} ", r, g, b),
  Collor::Hsv(h, s, v) => println!("HSV {} {} {} ", h, s, v),
}
```

```Rust
type IRPF = (f32, f32);
let renda = 3_000f32;

let tax: IRPF = 
match renda {
0..1_903.98	=> IRPF(0, 0), 
   1_903.99..2_826.65	=> IRPF(7.5, 142.80),
   2_826.66..3_751.05	=> IRPF(15.0, 354.80),
   3_751.06..4_664.68	=> IRPF(22.5, 636.13),
   4_664.68..         => IRPF(27.5,	869.36)
}
```

https://www.youtube.com/watch?v=8_HPKGZGM5I _em inglês_
