# Desafio Letras
Desafio prático para o processo seletivo da [Letras](https://www.letras.com/).


# O problema

## Descrição

​	Considere um jogo de formar palavras. Neste jogo, cada jogador recebe um conjunto
de letras e deve decidir qual palavra formada com aquelas letras vai contabilizar a maior
quantidade de pontos.
​	Pense nas letras que são disponibilizadas para cada jogador como "pecinhas" de um
jogo, ou seja, pode haver letras repetidas. Além disso, cada letra possui um valor, que ajuda
a contabilizar mais pontos na palavra que o jogador formar.
​	Para formar uma palavra, todas as letras que a compõem devem estar presentes no
conjunto de entrada. Em contrapartida, nem todas as letras disponíveis precisam ser
usadas. Por exemplo, se você possui as letras "ybttaaa", você pode formar a palavra
"batata", deixando de fora a letra "y".

## Pontuações 

* 1 ponto: E, A, I, O, N, R, T, L, S, U
* 2 pontos: D, G
* 3 pontos: B, C, M, P
* 5 pontos: F, H, V
* 8 pontos: J, X
* 13 pontos: Q, Z

## Regras

* O valor de cada letra é fixo e informado abaixo.

* O banco de palavras também é fixo e informado abaixo. Considere que não
  existem palavras que não estejam no banco.

* O valor da palavra corresponde à soma dos valores de cada letra que a
  compõem. O valor das letras que não foram utilizadas para formar a palavra
  não é descontado no processo.

* Em caso de empate no valor de duas palavras, a palavra mais curta deverá
  ser escolhida. Exemplo: "nada" (5 pontos) e "meu" (também 5 pontos) => a
  palavra "meu" deverá ser escolhida

* Se ainda assim houver empate, a palavra que vem primeiro em uma
  organização alfabética deve ser escolhida. Exemplo: "nada" (5 pontos) e
  "lado" (também 5 pontos) => a palavra “lado” deverá ser escolhida.

* Desconsiderar acentos e diferenças entre letras maiúsculas e minúsculas.

* Não copiar nenhuma solução de terceiros. Esperamos que você crie sua
  própria solução para o problema. Você está livre para acessar a Internet a
  fim de solucionar dúvidas relacionadas à plataforma para a qual você está
  desenvolvendo, estruturas de dados que você vier a utilizar, documentação
  da linguagem, etc.

  # Implementação

  ## Processo de compilação

  Esta solução foi feita em [Rust](https://www.rust-lang.org/). Para instalar uma *toolchain* de Rust, visite [rustup.rs](https://rustup.rs/) e siga as instruções. Uma vez feito isso:

  ```bash
  git clone https://github.com/vrmiguel/desafio-letras/ && cd desafio-letras
  cargo run --release # Compilará e executará o programa
  
  # Você pode também executar o binário diretamente:
  ./target/release/desafio-letras 
  
  cargo test # Executa a bateria de testes
  ```

  ## Algoritmo

### Processamento do banco de palavras

​	As palavras que compõem o banco de palavras dado na descrição do problema são representadas como elementos de um *array*, que é então iterado a fim de produzir variáveis do tipo [Word](src/word.rs).

​	Este tipo Word contém a *String* processada (isto é, a palavra dada em maiúsculo e sem caracteres especiais), o *score* da palavra e um *HashMap* de ocorrências de cada letra da palavra dada. Por ex., `"EGG"` terá um HashMap de ocorrências `{'E': 1, 'G': 2}`.

```rust
let words = words.iter().map(|word| Word::new(word));
```

​	As palavras serão então inseridas em um *min-heap*, representado através de um [BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.into_sorted_vec) da biblioteca padrão. 

​	Esta estrutura tem custo de inserção amortizado O(1) e é capaz de produzir um vetor ordenado de seus elementos em tempo ~O(n).

```rust
let mut word_bank = BinaryHeap::new();

for word in words {
	word_bank.push(word);
}

word_bank.into_sorted_vec()
```

​	Todo este procedimento será contido em uma variável estática preguiçosa (*lazy*), isto é, o banco de palavras será processado na primeira vez que sua variável for utilizada, tendo seu valor então disponibilizado em escopo global, podendo ser reutilizado sem que haja reprocessamento.

### REPL

​	O laço principal do programa lê uma palavra do `stdin` e gera sua `struct Word` correspondente.

​	Feito isso, buscamos a palavra de maior valor que conseguimos formar com as letras da palavra dada. Fazemos isso ao iterar o banco de palavras, que está ordenado de maneira decrescente, até encontrarmos a primeira palavra que pudermos formar, sendo desta forma um algoritmo linear.

```rust
pub fn get_best_move(word: &Word) -> Option<&'static Word> {
    for entry in WORD_BANK.iter() {
        if word.contains(&entry) {
            return Some(entry);
        }
    }
    None
}
```

Repetiremos este processo enquanto ainda houverem letras que podem formar palavras no banco de palavras.

### Função associadada `Word::contains`

`Word::contains` verifica se uma palavra (encapsulada em `Word`) *contém* outra, de maneira não sequencial. Por exemplo:

```rust
let ganglioside = Word::new("GANGLIOSIDE");
let egg = Word::new("EGG");
// Verdadeiro uma vez que a primeira palavra contém ao menos dois G e um E
assert_eq!(ganglioside.contains(&egg), true);
```

Este método utiliza os HashMaps de ocorrência de cada palavra, fazendo assim, no máximo, 26 comparações de modo linear.

```rust
pub fn contains(&self, other: &Word) -> bool {
    for (letter, count_in_other) in other.occurrences.iter() {
        let count = self.occurrences.get(letter);

        if count.is_none() {
            // A palavra `self` não contém uma letra de `other`,
            // portanto não pode a conter
            return false;
        }

        let count = count.unwrap();

        // Se `other` tiver uma quantidade maior de alguma letra do que `self` 
        // contém, então `self` não contém `other`.
        if count_in_other > count {
            return false;
        }
    }
    true
}
```

### Testes

O programa inclui testes para grande parte de sua lógica interna.

Para executar a bateria de teste:
```
cargo test
```

	running 18 tests
	test tests::best_move_tests::best_move_for_abcvoltdefaemg ... ok
	test tests::best_move_tests::best_move_for_queijinho ... ok
	test tests::diff_tests::diff_between_abacaxi_and_abaca ... ok
	test tests::best_move_tests::best_move_for_raxcaaai ... ok
	test tests::occurrence_count_tests::letters_in_abbey ... ok
	test tests::diff_tests::diff_between_deadly_and_dad ... ok
	test tests::occurrence_count_tests::letters_in_egg ... ok
	test tests::occurrence_count_tests::letters_in_potato ... ok
	test tests::ordering_tests::lado_bigger_than_nada ... ok
	test tests::ordering_tests::queijo_bigger_than_goiaba ... ok
	test tests::preprocessing_tests::diacritics ... ok
	test tests::preprocessing_tests::diacritics_and_mixed_casing ... ok
	test tests::preprocessing_tests::mixed_casing ... ok
	test tests::score_tests::test_every_letter ... ok
	test tests::score_tests::word_score_1 ... ok
	test tests::word_tests::ate_contains_tea ... ok
	test tests::word_tests::batarang_contains_bat ... ok
	test tests::word_tests::potato_does_not_contain_tomato ... ok

	test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

### Exemplos

```
Digite as letras disponíveis nesta jogada: bxiacajajaaj
ABACAXI, JA, JA, total de 36 pontos.
Sobraram: J
```

```
Digite as letras disponíveis nesta jogada: abc
Nenhuma palavra encontrada
```
