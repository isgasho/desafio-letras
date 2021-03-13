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

fn main() {
    let test = Word::new("QUEIJINHO");
    println!("Test word: {}", test);

    for word in WORD_BANK.iter() {
        if test.contains(&word) {
            println!("Best move: {}", word);
            break;
        }
    }
}
