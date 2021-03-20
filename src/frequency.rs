pub mod frequency {
    static ENG_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    const ENG_FREQS: [(char, f64); 26] = [
        ('a', 0.082),
        ('b', 0.015),
        ('c', 0.028),
        ('d', 0.043),
        ('e', 0.13),
        ('f', 0.022),
        ('g', 0.02),
        ('h', 0.061),
        ('i', 0.07),
        ('j', 0.0015),
        ('k', 0.0077),
        ('l', 0.04),
        ('m', 0.024),
        ('n', 0.067),
        ('o', 0.075),
        ('p', 0.019),
        ('q', 0.00095),
        ('r', 0.06),
        ('s', 0.063),
        ('t', 0.091),
        ('u', 0.028),
        ('v', 0.0098),
        ('w', 0.024),
        ('x', 0.0015),
        ('y', 0.02),
        ('z', 0.00074)
    ];

    pub fn eng_score(eng_passage:String) -> f64 {
        if eng_passage.len() == 0 { return 1.0 }
        let eng_passage: String = eng_passage
            .to_lowercase()
            .chars()
            .filter(char::is_ascii_lowercase)
            .collect();
        let mut eng_freqs: HashMap<char, f64> = HashMap::with_capacity(26);
        for (c, freq) in ENG_FREQS.iter() {
            eng_freqs.insert(*c, *freq);
        }
        let mut passage_freqs: HashMap<char, f64> = HashMap::with_capacity(26);
        let mut total = 0.0;
        for c in eng_passage.chars() {
            total += 1.0;
            let count = *passage_freqs.entry(c).or_insert(0.0);
            passage_freqs.insert(c, count + 1.0);
        }
        let mut dist: f64 = 0.0;
        for (c, _) in ENG_FREQS.iter() {
            let current_entry = *passage_freqs.entry(*c).or_insert(0.0);
            passage_freqs.insert(*c, current_entry / total);
            dist += (*passage_freqs.entry(*c).or_insert(0.0) - *eng_freqs.entry(*c).or_insert(0.0))
                .powf(2.0);
        }
        dist.sqrt()
    }
}
