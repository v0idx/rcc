//write lexer
mod tokens;

fn match_keyword(key: &str) -> u8 {
    for (k, v) in tokens::KEYWORDS.iter() {
        if &key == k {
            return *v;
        }
    }
    //need case for ident + constants
    return 0;
}

fn match_single(key: &str) -> u8 {
    for (k, v) in tokens::SINGLES.iter() {
        if &key == k {
            return *v;
        }
    }
    return 0;
}

fn match_double(key: &str) -> u8 {
    for (k, v) in tokens::DOUBLES.iter() {
        if &key == k {
            return *v;
        }
    }
    return 0;
}

fn match_key(key: &str) -> u8 {
    match key.len() {
        1 => return match_single(&key),
        2 => return match_double(&key),
        _ => return match_keyword(&key),
    }
}

fn consume_line_keywords(line: &str) -> Vec<u8> {
    let keys = line.split_whitespace();
    let mut line_toks = vec![];

    for k in keys {
        line_toks.push(match_key(k));
    }

    return line_toks;
}

pub fn lex(v: Vec<String>) -> Vec<u8> {
    let mut tok_v = vec![];
    //#[warn(unused_assignments)]
    let mut tok_l_v: Vec<u8>;
    for l in v {
        tok_l_v = consume_line_keywords(&l);
        for t in tok_l_v {
            tok_v.push(t);
        }
    }
    return tok_v;
}
