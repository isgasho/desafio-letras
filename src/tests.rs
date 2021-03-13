#[cfg(test)]
mod word_tests {
    use crate::word::Word;

    #[test]
    fn potato_does_not_contain_tomato() {
        let potato = Word::new("POTATO");
        let tomato = Word::new("TOMATO");

        assert!(!potato.contains(&tomato));
    }

    #[test]
    fn batarang_contains_bat() {
        let batarang = Word::new("BATARANG");
        let bat = Word::new("BAT");

        assert!(batarang.contains(&bat));
    }

    #[test]
    fn ate_contains_tea() {
        let ate = Word::new("ATE");
        let tea = Word::new("TEA");

        assert!(ate.contains(&tea));
    }
}

#[cfg(test)]
mod score_tests {
    use crate::word::Word;

    #[test]
    fn word_score_1() {
        assert_eq!(Word::calculate_score("BATATA"), 8);
    }

    #[test]
    fn test_every_letter() {
        assert_eq!(
            // Each of these letters should have score 1
            Word::calculate_score("EAIONRTLSU"),
            10 * 1
        );

        assert_eq!(
            // Each of these letters should have score 2
            Word::calculate_score("DG"),
            2 * 2
        );

        assert_eq!(
            // Each of these letters should have score 3
            Word::calculate_score("BCMP"),
            4 * 3
        );

        assert_eq!(
            // Each of these letters should have score 5
            Word::calculate_score("FHV"),
            3 * 5
        );

        assert_eq!(
            // Each of these letters should have score 2
            Word::calculate_score("JX"),
            8 * 2
        );

        assert_eq!(
            // Each of these letters should have score 2
            Word::calculate_score("QZ"),
            13 * 2
        );
    }
}

#[cfg(test)]
mod occurrence_count_tests {

    use crate::word::Word;
    use std::collections::HashMap;

    #[test]
    fn letters_in_potato() {
        let mut count: HashMap<char, u32> = HashMap::new();

        count.insert('P', 1);
        count.insert('O', 2);
        count.insert('T', 2);
        count.insert('A', 1);

        assert_eq!(count, Word::get_occurrence("POTATO"));
    }

    #[test]
    fn letters_in_egg() {
        let mut count: HashMap<char, u32> = HashMap::new();

        count.insert('E', 1);
        count.insert('G', 2);

        assert_eq!(count, Word::get_occurrence("EGG"));
    }

    #[test]
    fn letters_in_abbey() {
        let mut count: HashMap<char, u32> = HashMap::new();

        count.insert('A', 1);
        count.insert('B', 2);
        count.insert('E', 1);
        count.insert('Y', 1);

        assert_eq!(count, Word::get_occurrence("ABBEY"));
    }
}

#[cfg(test)]
mod ordering_tests {
    use crate::Word;

    #[test]
    fn lado_bigger_than_nada() {
        let lado = Word::new("lado");
        let nada = Word::new("nada");

        assert_eq!(
            // Reversing here since the ordering logic is reversed in
            // order to obtain a min-heap
            lado.cmp(&nada),
            std::cmp::Ordering::Greater
        );
    }
    #[test]
    fn queijo_bigger_than_goiaba() {
        let queijo = Word::new("queijo");
        let goiaba = Word::new("goiaba");

        assert_eq!(
            // Reversing here since the ordering logic is reversed in
            // order to obtain a min-heap
            queijo.cmp(&goiaba).reverse(),
            std::cmp::Ordering::Greater
        );
    }
}

#[cfg(test)]
mod best_move_tests {

    use crate::Word;
    use crate::get_best_move;

    #[test]
    fn best_move_for_queijinho() {
        let queijinho = Word::new("QUEIJINHO");

        assert_eq!(
            get_best_move(&queijinho).unwrap(),
            &Word::new("queijo")
        );
    }

    #[test]
    fn best_move_for_raxcaaai() {
        let raxcaaai = Word::new("raxcaaai");

        assert_eq!(
            get_best_move(&raxcaaai).unwrap(),
            &Word::new("xícara")
        );
    }

    #[test]
    fn best_move_for_abcvoltdefaemg() {
        let abcvoltdefaemg = Word::new("abcvoltdefaemg");

        assert_eq!(
            get_best_move(&abcvoltdefaemg).unwrap(),
            &Word::new("voLtaGem")
        );
    }
}
