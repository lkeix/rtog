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
