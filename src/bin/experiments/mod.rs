pub mod or;
pub mod xor;
use std::io::{self, Write};
use tars::*;
pub fn choose() -> Train {
    println!("temos os experimentos or,xor");
    println!("escolha um experimento escrevendo seu nome");
    io::stdout().flush();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice);
    match choice.trim() {
        "xor" => xor::train(),
        "or" => or::train(),
        //"local" => local::train(),
        _ => panic!(""),
    }
}
