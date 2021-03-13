/// TODO:
///     Replace unreachable! calls with Result and Option

use std::collections::{BinaryHeap, HashMap};

use lazy_static::lazy_static;

mod tests;
mod word;
mod repl;

use word::Word;

lazy_static! {
    static ref WORD_BANK: Vec<Word> = {
        let words = [
            "Abacaxi",
            "Manada",
            "mandar",
            "porta",
            "mesa",
            "Dado",
            "Mangas",
            "Já",
            "coisas",
            "radiografia",
            "matemática",
            "Drogas",
            "prédios",
            "implementação",
            "computador",
            "balão",
            "Xícara",
            "Tédio",
            "faixa",
            "Livro",
            "deixar",
            "superior",
            "Profissão",
            "Reunião",
            "Prédios",
            "Montanha",
            "Botânica",
            "Banheiro",
            "Caixas",
            "Xingamento",
            "Infestação",
            "Cupim",
            "Premiada",
            "empanada",
            "Ratos",
            "Ruído",
            "Antecedente",
            "Empresa",
            "Emissário",
            "Folga",
            "Fratura",
            "Goiaba",
            "Gratuito",
            "Hídrico",
            "Homem",
            "Jantar",
            "Jogos",
            "Montagem",
            "Manual",
            "Nuvem",
            "Neve",
            "Operação",
            "Ontem",
            "Pato",
            "Pé",
            "viagem",
            "Queijo",
            "Quarto",
            "Quintal",
            "Solto",
            "rota",
            "Selva",
            "Tatuagem",
            "Tigre",
            "Uva",
            "Último",
            "Vitupério",
            "Voltagem",
            "Zangado",
            "Zombaria",
            "Dor",
        ];

        let words = words.iter().map(|word| Word::new(word));

        let mut word_bank = BinaryHeap::new();

        for word in words {
            word_bank.push(word);
        }

        word_bank.into_sorted_vec()
    };
}

pub fn get_best_move(word: &Word) -> Option<&Word> {
    for entry in WORD_BANK.iter() {
        if word.contains(&entry) {
            return Some(entry);
        }
    }
    None
}

fn main() {
    let word = Word::new("hmmmm");
    let other_word = Word::new("hmm");

    let diff = word.get_occurrence_diff(&other_word);

    println!("Count: {:?}", word.occurrences);
    println!("New count: {:?}", diff);
}
