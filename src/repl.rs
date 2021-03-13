use std::io::{self, BufRead};

use colored::Colorize;

use crate::{Word, get_best_move};

pub fn start_loop() {
    let mut line = String::new();
    'repl: loop {
        print!("Digite as letras disponíveis nesta jogada:");
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("{}: houve um problema na leitura de palavra.", "erro".red());
            continue;
        }

        let word = Word::new(line.trim());
        match get_best_move(&word) {
            Some(best_move) => {
                println!("Best move: {}", best_move);
            },
            None => {
                println!("Nenhuma palavra encontrada.");
                // TODO: print letters remaining
            }
        }
    }
}