use lazy_static::lazy_static;
use std::collections::HashMap;

//doubles
const EQ: u16 = 0;
const NE: u16 = 1;
const GE: u16 = 2;
const LE: u16 = 3;

const AND: u16 = 4;
const OR: u16 = 5;

lazy_static! {
    pub static ref DOUBLES: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("==", EQ);
        m.insert("!=", NE);
        m.insert(">=", GE);
        m.insert("<=", LE);
        m.insert("&&", AND);
        m.insert("||", OR);
        m
    };
}

//singles
const LPAR: u16 = 100;
const RPAR: u16 = 101;
const LBRA: u16 = 102;
const RBRA: u16 = 103;
const LSQR: u16 = 104;
const RSQR: u16 = 105;

const EQUALS: u16 = 110;
const ADD: u16 = 111;
const SUB: u16 = 112;
const MUL: u16 = 113;
const DIV: u16 = 114;

const SEMI: u16 = 120;
const COMM: u16 = 121;

const LT: u16 = 130;
const GT: u16 = 131;
const NOT: u16 = 132;

lazy_static! {
    pub static ref SINGLES: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();

        m.insert("(", LPAR);
        m.insert(")", RPAR);
        m.insert("{", LBRA);
        m.insert("}", RBRA);
        m.insert("[", LSQR);
        m.insert("]", RSQR);
        m.insert("=", EQUALS);
        m.insert("+", ADD);
        m.insert("-", SUB);
        m.insert("*", MUL);
        m.insert("/", DIV);
        m.insert(";", SEMI);
        m.insert(",", COMM);
        m.insert("<", LT);
        m.insert(">", GT);
        m.insert("!", NOT);
        m
    };
}

//keywords

const INT: u16 = 200;
const IF: u16 = 201;
const WHILE: u16 = 202;
const FOR: u16 = 203;
const ELSE: u16 = 204;
const RET: u16 = 255;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();

        m.insert("int", INT);
        m.insert("if", IF);
        m.insert("while", WHILE);
        m.insert("for", FOR);
        m.insert("else", ELSE);
        m.insert("return", RET);
        m
    };
}
