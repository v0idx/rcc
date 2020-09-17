use std::collections::HashMap;

let whitespace = {' ','\t','\n','\r'};

let mut keywords = HashMap::new();

//todo
//need to make a tokeniser lol

//doubles
const EQ = 0;
const NE = 1;
const GE = 2;
const LE = 3;

const AND = 4;
const OR = 5;

let mut doubles = HashMap::new();

doubles.insert("==", EQ);
doubles.insert("!=", NE);
doubles.insert(">=", GE);
doubles.insert("<=", LE);
doubles.insert("&&", AND);
doubles.insert("||", OR);

//singles
const LPAR = 100;
const RPAR = 101;
const LBRA = 102;
const RBRA = 103;
const LSQR = 104;
const RSQR = 105;

const EQUALS = 110;
const ADD = 111;
const SUB = 112;
const MUL = 113;
const DIV = 114;

const SEMI = 120;
const COMM = 121;

const LT = 130;
const GT = 131;
const NOT = 132;

let mut singles = HashMap::new();

singles.insert("(", LPAR);
singles.insert(")", RPAR);
singles.insert("{", LBRA);
singles.insert("}", RBRA);
singles.insert("[", LSQR);
singles.insert("]", RSQR);

singles.insert("=", EQUALS);
singles.insert("+", ADD);
singles.insert("-", SUB);
singles.insert("*", MUL);
singles.insert("/", DIV);

singles.insert(";", SEMI);
singles.insert(",", COMM);

singles.insert("<", LT);
singles.insert(">", GT);
singles.insert("!", NOT);

//keywords

const INT = 200;
const IF = 201;
const WHILE = 202;
const FOR = 203;
const ELSE = 204;

const RET = 299;

let mut keywords = HashMap::new();

keywords.insert("int", INT);
keywords.insert("if", IF);
keywords.insert("while", WHILE);
keywords.insert("for",FOR);
keywords.insert("else", ELSE);
keywords.insert("return", RET);

