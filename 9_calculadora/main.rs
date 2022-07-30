// Este exemplo é uma contribuição de vários colegas da comunidade Rust do Telegram. Nos encontre lá 🦀
// Eu modifiquei este exemplo para trabalhar com hashmap + anonymous function. O exemplo inicial usava arrow function apenas.

use std::{io, collections::HashMap};

fn perform_operation(expression: &str, signal: char, op: impl Fn(f32, f32) -> f32) {
    let values: Vec<&str> = expression
        .split(signal)
        .map(|s| s.trim())
        .collect();
    let f = op(
        values[0].trim().parse::<f32>().unwrap(),
        values[1].trim().parse::<f32>().unwrap(),
    );
    println!("The result is ->: {}", f);
}

fn main() {
    println!("============================= Simples calculator Example =====================================");
    println!("Your operations need to follow the pattern as in the example: '5-2', '5+2', '5*2', '5/2'");

    let signals_aritimetic = ['+', '-', '/', '*'];
    
    let mut engine = HashMap::<char, fn(f32, f32) -> f32>::new();
    engine.insert('*', |x, y| x * y);
    engine.insert('/', |x, y| x / y);
    engine.insert('+', |x, y| x + y);
    engine.insert('-', |x, y| x - y);

    println!("Enter your operation: ");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to get operation");

    for &signal in signals_aritimetic.iter() {
        if let Some(_result) = operation.find(signal) {
            perform_operation(&operation, signal, engine[&signal])
        }
    }
}
