pub mod or;
pub mod xor;
use std::io::{self, Write};
use tars::*;
pub fn choose() -> Train {
    println!("temos os experimentos or,xor");
    println!("escolha um experimento escrevendo seu nome ou numero partindo de zero");
    io::stdout().flush();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice);
    match choice.trim() {
        "0" | "xor" => xor::train(),
        "1" | "or" => or::train(),
        _ => panic!(""),
    }
}
