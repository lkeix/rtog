use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

pub mod token_identifier {
  pub const ILLEGAL: &str = "ILLEGAL";
  pub const EOF: &str = "EOF";
  pub const COMMENT: &str = "COMMENT";

  pub const IDENT: &str = "IDENT";
  pub const INT: &str = "INT";
  pub const FLOAT: &str = "FLOAT";
  pub const IMAG: &str = "IMAG";
  pub const CHAR: &str = "CHAR";
  pub const STRING: &str = "STRING";

  pub const ADD: &str = "+";
  pub const SUB: &str = "-";
  pub const MUL: &str = "*";
  pub const QUO: &str = "/";
  pub const REM: &str = "%";

  pub const AND: &str = "&";
  pub const OR: &str = "|";
  pub const XOR: &str = "^";
  pub const SHL: &str = "<<";
  pub const SHR: &str = ">>";
  pub const AND_NOT: &str = "&^";

  pub const ADD_ASSIGN: &str = "+=";
  pub const SUB_ASSIGN: &str = "-=";
  pub const MUL_ASSIGN: &str = "*=";
  pub const QUO_ASSIGN: &str = "/=";
  pub const REM_ASSIGN: &str = "%=";

  pub const AND_ASSIGN: &str = "&=";
  pub const OR_ASSIGN: &str = "|=";
  pub const XOR_ASSIGN: &str = "^=";
  pub const SHL_ASSIGN: &str = "<<=";
  pub const SHR_ASSIGN: &str = ">>=";
  pub const AND_NOT_ASSIGN: &str = "&^=";

  pub const LAND: &str = "&&";
  pub const LOR: &str = "||";
  pub const ARROW: &str = "<-";
  pub const INC: &str = "++";
  pub const DEC: &str = "--";

  pub const EQL: &str = "==";
  pub const LSS: &str = "<";
  pub const GTR: &str = ">";
  pub const ASSIGN: &str = "=";
  pub const NOT: &str = "!";

  pub const NEQ: &str = "!=";
  pub const LEQ: &str = "<=";
  pub const GEQ: &str = ">=";
  pub const DEFINE: &str = ":=";
  pub const ELLIPSIS: &str = "...";

  pub const LPAREN: &str = "(";
  pub const LBRACK: &str = "[";
  pub const LBRACE: &str = "{";
  pub const COMMA: &str = ",";
  pub const PERIOD: &str = ".";

  pub const RPAREN: &str = ")";
  pub const RBRACK: &str = "]";
  pub const RBRACE: &str = "}";
  pub const SEMICOLON: &str = ";";
  pub const COLON: &str = ":";

  pub const BREAK: &str = "break";
  pub const CASE: &str = "case";
  pub const CHAN: &str = "chan";
  pub const CONST: &str = "const";
  pub const CONTINUE: &str = "continue";

  pub const DEFAULE: &str = "default";
  pub const DEFER: &str = "defer";
  pub const ELSE: &str = "else";
  pub const FALLTHROUGH: &str = "fallthrough";
  pub const FOR: &str = "for";

  pub const FUNC: &str = "func";
  pub const GO: &str = "go";
  pub const GOTO: &str = "goto";
  pub const IF: &str = "if";
  pub const IMPORT: &str = "import";

  pub const INTERFACE: &str = "interface";
  pub const MAP: &str = "map";
  pub const PACKAGE: &str = "package";
  pub const RANGE: &str = "range";
  pub const RETURN: &str = "return";

  pub const SELECT: &str = "select";
  pub const STRUCT: &str = "struct";
  pub const SWITCH: &str = "switch";
  pub const TYPE: &str = "type";
  pub const VAR: &str = "var";
  pub const TILDE: &str = "~";
}

const keyword_num: usize = 25;

#[derive(FromPrimitive)]
pub enum TokenIndex {
  ILLEGAL = 0,
  EOF,
  COMMENT,
  IDENT,
  INT,
  FLOAT,
  IMAG,
  CHAR,
  STRING,
  ADD,
  SUB,
  MUL,
  QUO,
  REM,
  AND,
  OR,
  XOR,
  SHL,
  SHR,
  AND_NOT,
  ADD_ASSIGN,
  SUB_ASSIGN,
  MUL_ASSIGN,
  QUO_ASSIGN,
  REM_ASSIGN,
  AND_ASSIGN,
  OR_ASSIGN,
  XOR_ASSIGN,
  SHL_ASSIGN,
  SHR_ASSIGN,
  AND_NOT_ASSIGN,
  LAND,
  LOR,
  ARROW,
  INC,
  DEC,
  EQL,
  LSS,
  GTR,
  ASSIGN,
  NOT,
  NEQ,
  LEQ,
  GEQ,
  DEFINE,
  ELLIPSIS,
  LPAREN,
  LBRACK,
  LBRACE,
  COMMA,
  PERIOD,
  RPAREN,
  RBRACK,
  RBRACE,
  SEMICOLON,
  COLON,

  BREAK,
  CASE,
  CHAN,
  CONST,
  CONTINUE,

  DEFAULE,
  DEFER,
  ELSE,
  FALLTHROUGH,
  FOR,
  FUNC,
  GO,
  GOTO,
  IF,
  IMPORT,

  INTERFACE,
  MAP,
  PACKAGE,
  RANGE,
  RETURN,
  SELECT,
  STRUCT,
  SWITCH,
  TYPE,
  VAR,

  TILDE
}

pub struct Token {
  pub tokens: Vec<String>,
  pub token: i64,
  pub keywords: HashMap<String, i64>
}

impl Token {
  pub fn new(self) -> Self {
    let tokens: Vec<String> = vec![
        String::from(token_identifier::ILLEGAL),
        String::from(token_identifier::EOF),
        String::from(token_identifier::COMMENT),
        String::from(token_identifier::IDENT),
        String::from(token_identifier::INT),
        String::from(token_identifier::FLOAT),
        String::from(token_identifier::IMAG),
        String::from(token_identifier::CHAR),
        String::from(token_identifier::STRING),
        String::from(token_identifier::ADD),
        String::from(token_identifier::SUB),
        String::from(token_identifier::MUL),
        String::from(token_identifier::QUO),
        String::from(token_identifier::REM),
        String::from(token_identifier::AND),
        String::from(token_identifier::OR),
        String::from(token_identifier::XOR),
        String::from(token_identifier::SHL),
        String::from(token_identifier::SHR),
        String::from(token_identifier::AND_NOT),
        String::from(token_identifier::ADD_ASSIGN),
        String::from(token_identifier::OR_ASSIGN),
        String::from(token_identifier::XOR_ASSIGN),
        String::from(token_identifier::MUL_ASSIGN),
        String::from(token_identifier::QUO_ASSIGN),
        String::from(token_identifier::AND_ASSIGN),
        String::from(token_identifier::OR_ASSIGN),
        String::from(token_identifier::XOR_ASSIGN),
        String::from(token_identifier::SHL_ASSIGN),
        String::from(token_identifier::SHR_ASSIGN),
        String::from(token_identifier::AND_NOT_ASSIGN),
        String::from(token_identifier::LAND),
        String::from(token_identifier::LOR),
        String::from(token_identifier::ARROW),
        String::from(token_identifier::INC),
        String::from(token_identifier::DEC),
        String::from(token_identifier::EQL),
        String::from(token_identifier::LSS),
        String::from(token_identifier::GTR),
        String::from(token_identifier::DEFINE),
        String::from(token_identifier::ELLIPSIS),
        String::from(token_identifier::LPAREN),
        String::from(token_identifier::LBRACK),
        String::from(token_identifier::LBRACE),
        String::from(token_identifier::COMMA),
        String::from(token_identifier::PERIOD),
        String::from(token_identifier::RPAREN),
        String::from(token_identifier::RBRACK),
        String::from(token_identifier::RBRACE),
        String::from(token_identifier::SEMICOLON),
        String::from(token_identifier::COLON),
        String::from(token_identifier::BREAK),
        String::from(token_identifier::CASE),
        String::from(token_identifier::CHAN),
        String::from(token_identifier::CONST),
        String::from(token_identifier::CONTINUE),
        String::from(token_identifier::DEFAULE),
        String::from(token_identifier::DEFER),
        String::from(token_identifier::ELSE),
        String::from(token_identifier::FALLTHROUGH),
        String::from(token_identifier::FOR),
        String::from(token_identifier::FUNC),
        String::from(token_identifier::GO),
        String::from(token_identifier::GOTO),
        String::from(token_identifier::IF),
        String::from(token_identifier::IMPORT),
        String::from(token_identifier::INTERFACE),
        String::from(token_identifier::MAP),
        String::from(token_identifier::PACKAGE),
        String::from(token_identifier::RANGE),
        String::from(token_identifier::RETURN),
        String::from(token_identifier::SELECT),
        String::from(token_identifier::STRUCT),
        String::from(token_identifier::SWITCH),
        String::from(token_identifier::TYPE),
        String::from(token_identifier::VAR),
        String::from(token_identifier::TILDE),
    ];
    let mut begin: usize = 0;
    let mut end: usize = 0;

    for i in 0..tokens.len() {
        if tokens[i] == token_identifier::BREAK {
            begin = i;
        }
        if tokens[i] == token_identifier::VAR {
            end = i;
        }
    }
    let mut keywords: HashMap<String, i64> = HashMap::with_capacity(end - begin);
    for i in begin..end+1 {
        keywords.insert(tokens[i].clone(), i as i64);
    }
    return Token {
      tokens: tokens,
      token: 0,
      keywords: keywords
    };
  }

  pub fn String(self) -> String {
      let tkn: usize = self.token as usize;
      let mut s: String = String::from("");
      if 0 <= self.token && tkn < self.tokens.len() {
        s = self.tokens.get(tkn).unwrap().to_string();
      }
      if s == "" {
          s = String::from("token(".to_string() + &(self.token as i64).to_string() + &")".to_string());
      }
      return s;
  }

  pub fn Precedence(self) -> i64 {
      let tkn: TokenIndex = FromPrimitive::from_i64(self.token).unwrap();
      match tkn {
          TokenIndex::LOR => {
              return 1
          },
          TokenIndex::LAND => {
              return 2
          },
          TokenIndex::EQL |
              TokenIndex::NEQ |
              TokenIndex::LSS |
              TokenIndex::LEQ |
              TokenIndex::GTR |
              TokenIndex::GEQ => {
                  return 3
              },
          TokenIndex::ADD |
              TokenIndex::SUB |
              TokenIndex::OR |
              TokenIndex::XOR => {
                  return 4
              },
          TokenIndex::MUL |
              TokenIndex::QUO |
              TokenIndex::REM |
              TokenIndex::SHL |
              TokenIndex::SHR |
              TokenIndex::AND |
              TokenIndex::AND_NOT => {
                  return 5
              },
          _ => {
              return 0
          }
      }
  }

  pub fn Lookup(self, ident: String) -> i64 {
      let mut ident_index = 0;
      for i in 0..self.tokens.len() {
          if self.tokens.get(i).unwrap() == token_identifier::IDENT {
              ident_index = i as i64;
          }
      }
      if !self.keywords.contains_key(&ident) {
          return ident_index;
      }
      return *self.keywords.get(&ident).unwrap();
  }

  pub fn is_literal(self) -> bool {
      let mut literal_begin: i64 = 0;
      let mut literal_end: i64 = 0;
      for i in 0..self.tokens.len() {
          if self.tokens.get(i).unwrap() == token_identifier::IDENT {
              literal_begin = i as i64;
          }
          if self.tokens.get(i).unwrap() == token_identifier::STRING {
              literal_end = i as i64;
          }
      }
      return literal_begin <= self.token && self.token <= literal_end;
  }
  pub fn is_operator(self) -> bool {
      let mut operator_begin: i64 = 0;
      let mut operator_end: i64 = 0;
      for i in 0..self.tokens.len() {
          if self.tokens.get(i).unwrap() == token_identifier::ADD {
              operator_begin = i as i64;
          }
          if self.tokens.get(i).unwrap() == token_identifier::COLON {
              operator_end = i as i64;
          }
      }
      return operator_begin <= self.token && self.token <= operator_end;
  }
}
