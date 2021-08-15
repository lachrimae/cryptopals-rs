use std::collections::HashMap;

#[derive(Debug)]
enum LexItem {
    Ampersand,
    EqualSign,
    Word(String),
}

// This function parses key-value pairs as in a URL parameter list.
// Special characters are '=' and '&', all other characters are treated
// as valid elements of a key or value.
pub fn parse_params(keys_and_vals: &String) -> Result<HashMap<String, String>, String> {
    let lexed = lex(keys_and_vals)?;
    Ok(parse(lexed)?)
}

fn parse(lexed: Vec<LexItem>) -> Result<HashMap<String, String>, String> {
    let mut it = lexed.iter().peekable();
    let mut nodes = HashMap::new();
    let mut first_time = true;
    while let Some(_) = it.peek() {
        if !first_time {
            match it.next() {
                Some(LexItem::Ampersand) => (),
                _ => return Err(format!("expected ampersand joining key-value pairs")),
            }
        }
        first_time = false;
        let key = match it.next() {
            Some(LexItem::Word(key)) => key.clone(),
            _ => return Err(format!("expected word as key")),
        };
        match it.next() {
            Some(LexItem::EqualSign) => (),
            _ => return Err(format!("expected equal sign joining key to value")),
        }
        let val = match it.next() {
            Some(LexItem::Word(val)) => val.clone(),
            _ => return Err(format!("expected word as key")),
        };
        nodes.insert(key, val);
    }
    Ok(nodes)
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    // rewrite this to lex many characters at once into a single LexItem::Word
    let mut count = 0;
    while let Some(&c) = it.peek() {
        match c {
            '&' => {
                result.push(LexItem::Ampersand);
                it.next();
            }
            '=' => {
                result.push(LexItem::EqualSign);
                it.next();
            }
            _ => {
                let mut word = Vec::new();
                while let Some(d) = it.next_if(|&x| x != '&' && x != '=') {
                    word.push(d);
                }
                let s: String = word.into_iter().collect();
                result.push(LexItem::Word(s));
            }
        }
        count = count + 1;
    }
    Ok(result)
}

pub fn profile_for(email: &str) -> String {
    let s: String = String::from(email)
        .chars()
        .filter(|&x| x != '=' && x != '&')
        .collect();
    format!("email={}&uid=10&role=user", s)
}
