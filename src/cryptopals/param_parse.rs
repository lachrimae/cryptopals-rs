use std::collections::HashMap;

// sequence -> pair&sequence | pair
// pair -> key=value
// key -> WORD
// value -> WORD

enum LexItem {
    Ampersand,
    EqualSign,
    Word(char),
}

struct ParseNode<'a> {
    pub key: &'a String,
    pub val: &'a String,
}

pub fn parseKV(keysAndVals:&String) -> HashMap<&str, &str> {
    let mut output = HashMap::new();


}

fn parse(lexed:&Vec<LexItem>) -> Vec<ParseNode> {
   // stuff has to happen here??? 
}

fn lex(input:&String) -> Vec<LexItem> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    // rewrite this to lex many characters at once into a single LexItem::Word
    while let Some(&c) = it.peek() {
        match c {
            '&' => { result.push(LexItem::Ampersand); }
            '=' => { result.push(LexItem::Ampersand); }
            _ => { result.push(LexItem::Word(c)); }
        }
    }
    result
}
