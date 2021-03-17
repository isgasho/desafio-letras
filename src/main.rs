/// https://github.com/vrmiguel/desafio-letras
/// Por favor leia o README.md para mais detalhes sobre a implementação

/// ​As palavras serão então inseridas em um min-heap, representado através de um BinaryHeap 
/// da biblioteca padrão.
/// ​Esta estrutura tem custo de inserção amortizado O(1) e é capaz de produzir um vetor ordenado de seus elementos em tempo O(n).
/// A busca da melhor palavra é feita de maneira linear, iterando o banco de palavras, que está ordenado de maneira decrescente


use std::collections::BinaryHeap;

use lazy_static::lazy_static;

mod repl;
mod tests;
mod word;
mod solution;

use word::Word;

lazy_static! {
    pub static ref WORD_BANK: Vec<Word> = {
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
    repl::start_loop();
}
