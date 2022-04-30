```Rust
let variable_name = value;            // no type specified
let variable_name:dataType = value;   //type specified
fn salary() {
    let fees = 25_000;
    let salary:f64 = 35_000.00;
    println!("fees is {} and salary is {}",fees,salary);
 }
```

## Immutable

By default, variables are immutable − read only in Rust. In other words, the variable's value cannot be changed once a value is bound to a variable name.

Let us understand this with an example

``` Rust
fn main() {
   let fees = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}
```

! Rowever you can override old reference using **let** again.

``` Rust
fn main(r) {
   let A = 3.1415;
   println!("Pi is {} ",fees);
   let Area = A * ( r * r)
   println!("Area of {r} is {A}",fees);
}
```

The output will be as shown below 

``` bash
error[E0384]: re-assignment of immutable variable `fees`
 --> main.rs:6:3
   |
 3 | let fees = 25_000;
   | ---- first assignment to `fees`
...
 6 | fees=35_000;
   | ^^^^^^^^^^^ re-assignment of immutable variable
error: aborting due to previous error(s)
```

The error message indicates the cause of the error – you cannot assign values twice to immutable variable fees. This is one of the many ways Rust allows programmers to write code and takes advantage of the safety and easy concurrency.

## Mutable

Variables are immutable by default. Prefix the variable name with mut keyword to make it mutable. The value of a mutable variable can be changed.

The syntax for declaring a mutable variable is as shown below −

``` Rust
let mut variable_name = value;
let mut variable_name:dataType = value;
Let us understand this with an example

fn main() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}
```

The output of the snippet is given below −

``` bash
fees is 25000
fees changed is 35000
```

## Variables - Complete guide

``` Rust
i8	u8
i16	u16
i32	u32
i64	u64
isize	usize
```