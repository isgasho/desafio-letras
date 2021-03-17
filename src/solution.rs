use std::collections::HashMap;

use crate::word::Word;
use crate::WORD_BANK;


pub fn get_best_move(word: &Word) -> Option<&'static Word> {
    for entry in WORD_BANK.iter() {
        if word.contains(&entry) {
            return Some(entry);
        }
    }
    None
}

pub fn get_solution(first_word: Word) -> (Vec<&'static Word>, HashMap<char, u32>) {
    let mut solution = vec![];

    let mut diff = HashMap::new();
    let mut word = first_word;
    let mut best_move = get_best_move(&word.clone());

    while !best_move.is_none() {
        let best_move_word = best_move.unwrap();
        solution.push(best_move_word);
        diff = word.get_occurrence_diff(best_move_word);
        word = Word::from(diff.clone());
        best_move = get_best_move(&word.clone());
    }

    (solution, diff)
}