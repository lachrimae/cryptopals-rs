use std::collections::HashMap;

pub const ENG_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
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
    ('z', 0.00074),
];

pub fn eng_score(eng_passage: String, _euclidean: bool) -> f64 {
    assert_ne!(eng_passage.len(), 0);
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
        let delta = *passage_freqs.entry(*c).or_insert(0.0) - *eng_freqs.entry(*c).or_insert(0.0);
        dist += delta.abs();
    }
    dist
}

pub fn eng_score_old(eng_passage: String, euclidean: bool) -> f64 {
    assert_ne!(eng_passage.len(), 0);
    let punct_cost = 1.0
        * (eng_passage
            .chars()
            .filter(|c| !c.is_ascii())
            .collect::<String>()
            .len() as f64);
    let no_spaces_present = eng_passage
        .chars()
        .filter(|c| *c == ' ')
        .collect::<String>()
        .len()
        == 0;
    let no_spaces_cost = if no_spaces_present { 5.0 } else { 0.0 };
    let uppercase_rate = eng_passage
        .chars()
        .filter(char::is_ascii_uppercase)
        .collect::<String>()
        .len() as f64
        / eng_passage.len() as f64;
    let uppercase_cost = if uppercase_rate > 0.2 { 3.0 } else { 0.0 };
    let eng_passage: String = eng_passage
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_lowercase)
        .collect();
    if eng_passage.len() == 0 {
        return 20.0;
    }
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
        let incr = *passage_freqs.entry(*c).or_insert(0.0) - *eng_freqs.entry(*c).or_insert(0.0);
        let incr = if euclidean {
            incr.powf(2.0)
        } else {
            incr.abs()
        };
        dist += incr;
    }
    uppercase_cost + punct_cost + no_spaces_cost + if euclidean { dist.sqrt() } else { dist }
}
