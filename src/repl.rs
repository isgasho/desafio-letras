use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{solution::get_solution, Word};

fn print_letters_left(occurrences: HashMap<char, u32>) {
    let mut letters = Vec::with_capacity(5);

    for (letter, count) in occurrences {
        match count {
            0 => continue,
            1 => letters.push(letter),
            count => {
                for _ in 0..count {
                    letters.push(letter);
                }
            }
        }
    }

    let len = letters.len();

    letters
        .into_iter()
        .enumerate()
        .for_each(|(idx, letter)| {
            if idx == len - 1 {
                print!("{}\n", letter);
            } else {
                print!("{}, ", letter)
            }
        }
    );
}

pub fn start_loop() {
    let mut line = String::new();
    loop {
        eprint!("Digite as letras disponíveis nesta jogada: ");

        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("erro: houve um problema na leitura de palavra.");
            continue;
        }

        let word = Word::new(line.trim());
        line.clear();

        let (solution, letters_left) = get_solution(word);

        match solution.len() {
            0 => println!("Nenhuma palavra encontrada"),
            1 => {
                let word = solution[0];
                println!("{}, palavra de {} pontos.", word.text, word.score);
            }
            _ => {
                for word in solution.iter() {
                    print!("{}, ", word.text)
                }
                let sum: u32 = solution.iter().map(|word| word.score).sum();
                print!("total de {} pontos.\n", sum);
            }
        }

        if !letters_left.is_empty() {
            print!("Sobraram: ");
            print_letters_left(letters_left);
        }
        println!();

        io::stdout().flush().unwrap();
    }
}
