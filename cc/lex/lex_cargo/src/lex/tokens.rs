use lazy_static::lazy_static;
use std::collections::HashMap;

//doubles
const EQ: u8 = 0;
const NE: u8 = 1;
const GE: u8 = 2;
const LE: u8 = 3;

const AND: u8 = 4;
const OR: u8 = 5;

lazy_static! {
    pub static ref DOUBLES: HashMap<&'static str, u8> = {
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
const LPAR: u8 = 100;
const RPAR: u8 = 101;
const LBRA: u8 = 102;
const RBRA: u8 = 103;
const LSQR: u8 = 104;
const RSQR: u8 = 105;

const EQUALS: u8 = 110;
const ADD: u8 = 111;
const SUB: u8 = 112;
const MUL: u8 = 113;
const DIV: u8 = 114;

const SEMI: u8 = 120;
const COMM: u8 = 121;

const LT: u8 = 130;
const GT: u8 = 131;
const NOT: u8 = 132;

lazy_static! {
    pub static ref SINGLES: HashMap<&'static str, u8> = {
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

const INT: u8 = 200;
const IF: u8 = 201;
const WHILE: u8 = 202;
const FOR: u8 = 203;
const ELSE: u8 = 204;
const RET: u8 = 255;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, u8> = {
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
