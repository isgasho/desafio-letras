use std::collections::BinaryHeap;

use lazy_static::lazy_static;

mod tests;
mod word;

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
            // println!("Best move: {}", entry);
            return Some(entry);
        }
    }
    None
}

fn main() {
    let test = Word::new("abcvoltdefaemg");
    println!("Test word: {:?}", get_best_move(&test));
}
