use std::io;
use rand::seq::IndexedRandom;

fn main() {
    
    println!("Enter the choices (comma separated):");
    let mut choices = String::new();
    io::stdin()
        .read_line(&mut choices)
        .expect("Failed to read choices.");

    let choices: Vec<&str> = choices.split(",").collect();
    match choices.choose(&mut rand::rng()) {
        Some(i) => println!("\n{} was chosen.", i.trim()),
        None    => println!("Nothing was chosen!")
    }
}
