//TODO
//need to parse for longest token
mod files;
mod tok;

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

fn match_ident(line: &str) -> u16 {
    return 0;
}

fn match_double(line: &str) -> u16 {
    let reserved: Vec<char> = vec!['{', '}', '=', '!', '-', '|', '&', '>', '<', '+'];
    let chars = line.chars();
    let mut v: Vec<char> = vec![];

    println!("line: {}", &line);

    for c in chars {
        println!("char: {}", &c);
        if reserved.contains(&c) {
            v.push(c);
        } else {
            //if not a reserved then an ident
            match_ident(line);
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
    let reserved: Vec<char> = vec!['{', '}', '=', '!', '-', '|', '&', '>', '<', '+'];

    //need to like, fix this lmao

    while !(lines.is_empty()) {
        let curr_line = lines.remove(0);
        for i in 0..curr_line.len() {
            if reserved.contains(&curr_line.chars().nth(i).unwrap()) {
                println!("currchar: {}", &curr_line.chars().nth(i).unwrap());
                v.push(match_double(&curr_line));
            }
        }
        // else {
        //     //if not reserved then has to be keyword or identifier
        // }
    }

    return v;
}

pub fn lex(file_path: &str) {
    let to_lex: Vec<String> = files::file_in(file_path);

    // for s in &to_lex {
    //     println!("{}", s);
    // }

    let lexed: Vec<u16> = parse(to_lex);

    for u in &lexed {
        println!("{}", u);
    }
}
