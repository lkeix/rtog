#[cfg(test)]
extern crate rstest;
extern crate speculate;

use rstest::*;
use rtog::token::token::token_identifier::*;
use rtog::token::token::Token;
use speculate::speculate;
use std::collections::HashMap;

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
      let tkn: Token = Token{
        token: 0,
        tokens: vec![],
        keywords: HashMap::new()
      }.new();
      assert_eq!(ILLEGAL, tkn.String());
    }
  }

  describe "check precedence" {
    #[rstest]
    fn test_precedenence_lor() {
        let mut tkn: Token = Token{
            token: 0,
            tokens: vec![],
            keywords: HashMap::new()
        }.new();
        tkn.token = 32; // 32 is LOR in TokenIndex
        assert_eq!(1, tkn.Precedence());
    }

    #[rstest]
    fn test_precedenence_land() {
        let mut tkn: Token = Token{
            token: 0,
            tokens: vec![],
            keywords: HashMap::new()
        }.new();
        tkn.token = 31;
        assert_eq!(2, tkn.Precedence());
    }

    fn gen_token() -> Token {
        return Token{
            token: 0,
            tokens: vec![],
            keywords: HashMap::new()
        }.new();
    }

    #[rstest]
    fn test_precedenence_eql_neq_lss_leq_gtr_geq() {
        let mut tkn: Token = gen_token();
        tkn.token = 36;
        let eql: i64 = tkn.Precedence(); 
        let mut tkn: Token = gen_token();
        tkn.token = 41;
        let neq: i64 = tkn.Precedence();
        let mut tkn: Token = gen_token();
        tkn.token = 37;
        let lss: i64 = tkn.Precedence();
        let mut tkn: Token = gen_token();
        tkn.token = 42;
        let leq: i64 = tkn.Precedence();
        let mut tkn: Token = gen_token();
        tkn.token = 38;
        let gtr: i64 = tkn.Precedence();
        let mut tkn: Token = gen_token();
        tkn.token = 43;
        let geq: i64 = tkn.Precedence();

        assert_eq!(3, eql);
        assert_eq!(3, neq);
        assert_eq!(3, lss);
        assert_eq!(3, leq);
        assert_eq!(3, gtr);
        assert_eq!(3, geq);
    }
  }

  fn gen_token() -> Token {
      return Token{
          token: 0,
          tokens: vec![],
          keywords: HashMap::new()
      }.new();
  }

  #[rstest]
  fn test_precedence_add_sub_or_xor() {
      let mut tkn: Token = gen_token();
      tkn.token = 9;
      let add: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 10;
      let sub: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 15;
      let or: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 16;
      let xor: i64 = tkn.Precedence();

      assert_eq!(4, add);
      assert_eq!(4, sub);
      assert_eq!(4, or);
      assert_eq!(4, xor);
  }


  #[rstest]
  fn test_precedence_mul_quo_rem_shl_shr_and_andnot() {
      let mut tkn: Token = gen_token();
      tkn.token = 11;
      let mul: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 12;
      let quo: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 13;
      let rem: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 17;
      let shl: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 18;
      let shr: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 14;
      let and: i64 = tkn.Precedence();
      let mut tkn: Token = gen_token();
      tkn.token = 19;
      let andnot: i64 = tkn.Precedence();

      assert_eq!(5, mul);
      assert_eq!(5, quo);
      assert_eq!(5, rem);
      assert_eq!(5, shl);
      assert_eq!(5, shr);
      assert_eq!(5, and);
      assert_eq!(5, andnot);
  }

  #[rstest]
  fn test_new() {
      let mut tkn: Token = Token{
          token: 0,
          tokens: vec![],
          keywords: HashMap::new()
      }.new();

      let ans_tokens: Vec<String> = vec![
          BREAK.to_string(),
          CASE.to_string(),
          CHAN.to_string(),
          CONST.to_string(),
          CONTINUE.to_string(),
          DEFAULE.to_string(),
          DEFER.to_string(),
          ELSE.to_string(),
          FALLTHROUGH.to_string(),
          FOR.to_string(),
          FUNC.to_string(),
          GO.to_string(),
          GOTO.to_string(),
          IF.to_string(),
          IMPORT.to_string(),
          INTERFACE.to_string(),
          MAP.to_string(),
          PACKAGE.to_string(),
          RANGE.to_string(),
          RETURN.to_string(),
          SELECT.to_string(),
          STRUCT.to_string(),
          SWITCH.to_string(),
          TYPE.to_string(),
          VAR.to_string()
      ];

      let mut tokens: Vec<String> = vec![];
      for (keyword, i) in tkn.keywords {
          tokens.push(keyword);
          println!("{}", i);
      }
      tokens.sort();
      for i in 0..ans_tokens.len() {
          assert_eq!(ans_tokens[i], tokens[i]);
      }
  }

  #[rstest]
  fn test_lookup() {
      let tkn: Token = gen_token();
      let brk_idx: i64 = tkn.Lookup("break".to_string());
      let tkn: Token = gen_token();
      let case_idx: i64 = tkn.Lookup("case".to_string());
      let tkn: Token = gen_token();
      let chan_idx: i64 = tkn.Lookup("chan".to_string());
      let tkn: Token = gen_token();
      let const_idx: i64 = tkn.Lookup("const".to_string());
      let tkn: Token = gen_token();
      let continue_idx: i64 = tkn.Lookup("continue".to_string());
      let tkn: Token = gen_token();
      let default_idx: i64 = tkn.Lookup("default".to_string());
      let tkn: Token = gen_token();
      let defer_idx: i64 = tkn.Lookup("defer".to_string());
      let tkn: Token = gen_token();
      let else_idx: i64 = tkn.Lookup("else".to_string());
      let tkn: Token = gen_token();
      let fallthrough_idx: i64 = tkn.Lookup("fallthrough".to_string());
      let tkn: Token = gen_token();
      let for_idx: i64 = tkn.Lookup("for".to_string());
      let tkn: Token = gen_token();
      let func_idx: i64 = tkn.Lookup("func".to_string());
      let tkn: Token = gen_token();
      let go_idx: i64 = tkn.Lookup("go".to_string());
      let tkn: Token = gen_token();
      let goto_idx: i64 = tkn.Lookup("goto".to_string());
      let tkn: Token = gen_token();
      let if_idx: i64 = tkn.Lookup("if".to_string());
      let tkn: Token = gen_token();
      let import_idx: i64 = tkn.Lookup("import".to_string());
      let tkn: Token = gen_token();
      let interface_idx: i64 = tkn.Lookup("interface".to_string());
      let tkn: Token = gen_token();
      let map_idx: i64 = tkn.Lookup("map".to_string());
      let tkn: Token = gen_token();
      let package_idx: i64 = tkn.Lookup("package".to_string());
      let tkn: Token = gen_token();
      let range_idx: i64 = tkn.Lookup("range".to_string());
      let tkn: Token = gen_token();
      let return_idx: i64 = tkn.Lookup("return".to_string());
      let tkn: Token = gen_token();
      let select_idx: i64 = tkn.Lookup("select".to_string());
      let tkn: Token = gen_token();
      let struct_idx: i64 = tkn.Lookup("struct".to_string());
      let tkn: Token = gen_token();
      let switch_idx: i64 = tkn.Lookup("switch".to_string());
      let tkn: Token = gen_token();
      let type_idx: i64 = tkn.Lookup("type".to_string());
      let tkn: Token = gen_token();
      let var_idx: i64 = tkn.Lookup("var".to_string());

      assert_eq!(51, brk_idx);
      assert_eq!(52, case_idx);
      assert_eq!(53, chan_idx);
      assert_eq!(54, const_idx);
      assert_eq!(55, continue_idx);
      assert_eq!(56, default_idx);
      assert_eq!(57, defer_idx);
      assert_eq!(58, else_idx);
      assert_eq!(59, fallthrough_idx);
      assert_eq!(60, for_idx);
      assert_eq!(61, func_idx);
      assert_eq!(62, go_idx);
      assert_eq!(63, goto_idx);
      assert_eq!(64, if_idx);
      assert_eq!(65, import_idx);
      assert_eq!(66, interface_idx);
      assert_eq!(67, map_idx);
      assert_eq!(68, package_idx);
      assert_eq!(69, range_idx);
      assert_eq!(70, return_idx);
      assert_eq!(71, select_idx);
      assert_eq!(72, struct_idx);
      assert_eq!(73, switch_idx);
      assert_eq!(74, type_idx);
      assert_eq!(75, var_idx);
  }

  #[rstest]
  fn test_is_literal() {
      let mut tkn: Token = gen_token();
      let mut ident: i64 = 0;
      let mut add: i64 = 0;
      for (i, token) in tkn.tokens.iter().enumerate() {
          if token == IDENT {
              ident = i as i64;
          }
          if token == ADD {
              add = i as i64;
          }
      }
      tkn.token = ident;
      assert_eq!(tkn.is_literal(), true);
      let mut tkn: Token = gen_token();
      tkn.token = add;
      assert_eq!(tkn.is_literal(), false);
  }
}
