use std::io::{self, BufRead};

use colored::Colorize;

use crate::{Word, get_solution};

pub fn start_loop() {
    let mut line = String::new();
    loop {
        eprint!("Digite as letras disponíveis nesta jogada: ");
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("{}: houve um problema na leitura de palavra.", "erro".red());
            continue;
        }
        let word = Word::new(line.trim());
        line.clear();

        let (solution, words_left) = get_solution(word);

        match solution.len() {
            0 => println!("Nenhuma palavra encontrada"),
            1 => {
                let word = solution[0];
                println!("{}, palavra de {} pontos.", word.word, word.score);
            },
            _ => {
                solution.iter().for_each(|word|
                    print!("{} ", word.word)
                );
                let sum: u32 = solution.iter().map(|word| word.score).sum();
            }
        }

        // match get_best_move(&word) {
        //     Some(best_move) => {
        //         println!("Best move: {}", best_move);
        //     },
        //     None => {
        //         println!("Nenhuma palavra encontrada.");
        //         // TODO: print letters remaining
        //     }
        // }
    }
}