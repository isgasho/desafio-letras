use std::cmp::Reverse;
use std::{cmp, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    pub word: String,
    pub occurrences: HashMap<char, u32>,
    pub score: u32,
}

/// Defines ordering for a word.
/// Ordering is reversed only for the sake of creating a min-heap through
/// use of std's BinaryHeap
impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // As per the project's specification, words are ordered through their scores,
        // except when their scores are the same, in which case the word with smallest length is preferred.
        // If there's still a tie, the word that comes first in alphabetical order is to be chosen

        // Checks for the smallest string between two words
        let tie_breaker = |word: &Word, other_word| {
            let self_word_len = word.word.len();
            let other_word_len = other.word.len();

            match self_word_len.cmp(&other_word_len) {
                std::cmp::Ordering::Equal => word.word.cmp(&other.word),
                other => other,
            }
        };

        let score_ordering = self.score.cmp(&other.score);
        if score_ordering == std::cmp::Ordering::Equal {
            tie_breaker(self, other)
        } else {
            score_ordering
        }
        .reverse()
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Word: {}\nOccurrences: {:#?}, score: {}",
            self.word, self.occurrences, self.score
        )
    }
}

impl From<HashMap<char, u32>> for Word {
    fn from(occurrences: HashMap<char, u32>) -> Self {
        let word = occurrences
            .iter()
            .map(|(&char, &count)| char.to_string().repeat(count as usize))
            .fold("".into(), |res, val| format!("{}{}", res, val));

        Self {
            occurrences: Word::get_occurrence(&word),
            score: Word::calculate_score(&word),
            word,
        }
    }
}

impl Word {
    pub fn new(word: &str) -> Self {
        let word = Word::preprocess_word(word);
        Self {
            occurrences: Word::get_occurrence(&word),
            score: Word::calculate_score(&word),
            word,
        }
    }

    pub fn preprocess_word(word: &str) -> String {
        let letters = word.chars().map(String::from);
        letters
            .map(|j| match j.as_ref() {
                "À" | "Á" | "Â" | "Ã" | "A" | "à" | "á" | "â" | "ã" | "a" => "A",

                "b" | "B" => "B",

                "ç" | "Ç" | "c" | "C" => "C",

                "d" | "D" => "D",

                "È" | "É" | "Ê" | "E" | "è" | "é" | "ê" | "e" => "E",

                "f" | "F" => "F",

                "g" | "G" => "G",

                "h" | "H" => "H",

                "Ì" | "Í" | "Î" | "I" | "ì" | "í" | "î" | "i" => "I",

                "j" | "J" => "J",

                "k" | "K" => "K",

                "l" | "L" => "L",

                "m" | "M" => "M",

                "n" | "N" => "N",

                "Ò" | "Ó" | "Ô" | "Õ" | "O" | "ò" | "ó" | "ô" | "õ" | "o" => "O",

                "p" | "P" => "P",

                "q" | "Q" => "Q",

                "r" | "R" => "R",

                "s" | "S" => "S",

                "t" | "T" => "T",

                "Ú" | "U" | "ú" | "u" => "U",

                "v" | "V" => "V",

                "x" | "X" => "X",

                "z" | "Z" => "Z",

                "y" | "Y" => "Y",

                // Whitespace
                "\u{200b}" => "",
                "\n" => "",

                other => {
                    // Should be unreachable as per the problem description
                    println!("Unreachable: {:?}", other);
                    unreachable!();
                }
            })
            .collect()
    }

    /// `contains` checks if a word contains all of the letters of another word
    pub fn contains(&self, other: &Word) -> bool {
        for (letter, count_in_other) in other.occurrences.iter() {
            let count = self.occurrences.get(letter);

            // If the current letter wasn't found on the other word,
            // TODO
            if count.is_none() {
                return false;
            }

            let count = count.unwrap();

            if count_in_other > count {
                // If the other word has a higher count of a given word, then `self` cannot contain the other word.
                return false;
            }
        }
        true
    }

    /// Gets the difference between the occurrence hashmaps of two words
    /// Note: this function assumes that first.contains(&second) == true
    pub fn get_occurrence_diff(&self, second: &Word) -> HashMap<char, u32> {
        let mut occurrences = self.occurrences.clone();

        for (letter, count) in second.occurrences.clone() {
            occurrences.entry(letter).and_modify(|c| *c -= count);

            let new_count = occurrences.get(&letter).unwrap();
            if new_count == &0 {
                occurrences.remove_entry(&letter);
            }
        }

        occurrences
    }

    pub fn get_occurrence(word: &str) -> HashMap<char, u32> {
        let mut counts: HashMap<char, u32> = HashMap::new();

        let chars = word.chars();

        for char in chars {
            *counts.entry(char).or_insert(0) += 1;
        }

        counts
    }

    pub fn calculate_score(word: &str) -> u32 {
        let get_letter_value = |letter| {
            match letter {
                // The value of these two letters aren't defined in the project's
                // description, so I'm assuming they have value 0.
                'Y' | 'K' => 0,

                'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
                'D' | 'G' => 2,
                'B' | 'C' | 'M' | 'P' => 3,
                'F' | 'H' | 'V' => 5,
                'J' | 'X' => 8,
                'Q' | 'Z' => 13,
                _other => {
                    // Unreachable according to the problem description
                    unreachable!()
                }
            }
        };

        word.chars().map(get_letter_value).sum()
    }
}
