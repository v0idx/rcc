//TODO
//need to parse for longest token
mod files;
mod tok;

static reserved: Vec<char> = vec!['{', '}', '=', '!', '-', '|', '&', '>', '<'];

fn join(v: &Vec<char>) -> String {
    let mut ret_string: String = String::from("");

    for c in v {
        ret_string.push(*c);
    }

    return ret_string;
}

fn lex_single(key: &str) -> u16 {
    for (k, v) in tok::SINGLES.iter() {
        if &key == k {
            return *v;
        }
    }

    return 0;
}

fn lex_double(key: &str) -> u16 {
    for (k, v) in tok::DOUBLES.iter() {
        if &key == k {
            return *v;
        }
    }

    return 0;
}

fn match_double(line: &str) -> u16 {
    let chars = line.chars();
    let mut v: Vec<char> = vec![];

    for c in chars {
        if reserved.contains(&c) {
            v.push(c);
        } else {
            break;
        }
    }

    //got token, check length
    let curr_tok: String = join(&v);

    match v.len() {
        1 => return lex_single(&curr_tok),
        2 => return lex_double(&curr_tok),
        _ => return 0,
    }
}

fn parse(mut lines: Vec<String>) -> Vec<u16> {
    let mut v = vec![];

    while !(v.is_empty()) {
        let curr_line = lines.remove(0);
        if reserved.contains(&curr_line.chars().nth(0).unwrap()) {
            match_double(&curr_line);
        } else {
            //if not reserved then has to be keyword or identifier
        }
    }

    return v;
}

pub fn lex(file_path: &str) {
    let to_lex: Vec<String> = files::file_in(file_path);

    for s in &to_lex {
        println!("{}", s);
    }
}
