#[cfg(test)]
extern crate rstest;
extern crate speculate;

use rstest::*;
use rtog::token::token::token_identifier::*;
use rtog::token::token::Token;
use speculate::speculate;

speculate! {
  describe "Check constant tokens"  {
    #[rstest]
    fn test_constant_token() {
      assert_eq!(ILLEGAL, "ILLEGAL");
      assert_eq!(EOF, "EOF");
      assert_eq!(COMMENT, "COMMENT");
      assert_eq!(IDENT, "IDENT");
      assert_eq!(INT, "INT");
      assert_eq!(FLOAT, "FLOAT");
      assert_eq!(IMAG, "IMAG");
      assert_eq!(CHAR, "CHAR");
      assert_eq!(STRING, "STRING");

      assert_eq!(ADD, "+");
      assert_eq!(SUB, "-");
      assert_eq!(MUL, "*");
      assert_eq!(QUO, "/");
      assert_eq!(REM, "%");

      assert_eq!(AND, "&");
      assert_eq!(OR, "|");
      assert_eq!(XOR, "^");
      assert_eq!(SHL, "<<");
      assert_eq!(SHR, ">>");
      assert_eq!(AND_NOT, "&^");

      assert_eq!(ADD_ASSIGN, "+=");
      assert_eq!(SUB_ASSIGN, "-=");
      assert_eq!(MUL_ASSIGN, "*=");
      assert_eq!(QUO_ASSIGN, "/=");
      assert_eq!(REM_ASSIGN, "%=");

      assert_eq!(AND_ASSIGN, "&=");
      assert_eq!(OR_ASSIGN, "|=");
      assert_eq!(XOR_ASSIGN, "^=");
      assert_eq!(SHL_ASSIGN, "<<=");
      assert_eq!(SHR_ASSIGN, ">>=");
      assert_eq!(AND_NOT_ASSIGN, "&^=");

      assert_eq!(LAND, "&&");
      assert_eq!(LOR, "||");
      assert_eq!(ARROW, "<-");
      assert_eq!(INC, "++");
      assert_eq!(DEC, "--");

      assert_eq!(EQL, "==");
      assert_eq!(LSS, "<");
      assert_eq!(GTR, ">");
      assert_eq!(ASSIGN, "=");
      assert_eq!(NOT, "!");

      assert_eq!(NEQ, "!=");
      assert_eq!(LEQ, "<=");
      assert_eq!(GEQ, ">=");
      assert_eq!(DEFINE, ":=");
      assert_eq!(ELLIPSIS, "...");

      assert_eq!(LPAREN, "(");
      assert_eq!(LBRACK, "[");
      assert_eq!(LBRACE, "{");
      assert_eq!(COMMA, ",");
      assert_eq!(PERIOD, ".");

      assert_eq!(RPAREN, ")");
      assert_eq!(RBRACK, "]");
      assert_eq!(RBRACE, "}");
      assert_eq!(SEMICOLON, ";");
      assert_eq!(COLON, ":");

      assert_eq!(BREAK, "break");
      assert_eq!(CASE, "case");
      assert_eq!(CHAN, "chan");
      assert_eq!(CONST, "const");
      assert_eq!(CONTINUE, "continue");

      assert_eq!(DEFAULE, "default");
      assert_eq!(DEFER, "defer");
      assert_eq!(ELSE, "else");
      assert_eq!(FALLTHROUGH, "fallthrough");
      assert_eq!(FOR, "for");

      assert_eq!(FUNC, "func");
      assert_eq!(GO, "go");
      assert_eq!(GOTO, "goto");
      assert_eq!(IF, "if");
      assert_eq!(IMPORT, "import");

      assert_eq!(INTERFACE, "interface");
      assert_eq!(MAP, "map");
      assert_eq!(PACKAGE, "package");
      assert_eq!(RANGE, "range");
      assert_eq!(RETURN, "return");

      assert_eq!(SELECT, "select");
      assert_eq!(STRUCT, "struct");
      assert_eq!(SWITCH, "switch");
      assert_eq!(TYPE, "type");
      assert_eq!(VAR, "var");
      assert_eq!(TILDE, "~");
    }
  }
  describe "check token string" {
    #[rstest]
    fn test_token_string() {
      let tkn = Token{
        token: 1,
        tokens: vec![],
      }.new();
      assert_eq!("", tkn.String());
    }
  }
}
