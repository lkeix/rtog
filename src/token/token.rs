pub mod token_identifier {
  pub const ILLEGAL: &str = "ILLEGAL";
  pub const EOF: &str = "EOF";
  pub const COMMENT: &str = "COMMENT";

  pub const IDENT: &str = "IDENT";
  pub const INT: &str = "INT";
  pub const FLOAT: &str = "FLOAT";
  pub const IMAG: &str = "IMAG";
  pub const CHAR: &str = "CHAR";
  pub const STRING: &str = "&strING";

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
}
