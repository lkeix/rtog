#[cfg(test)]
extern crate rstest;
extern crate speculate;

use rstest::*;
use rtog;
use speculate::speculate;

speculate! {
  describe "Check constant tokens"  {
    #[rstest]
    fn test_constant_token() {
      assert_eq!(rtog::token::token::token_identifier::ILLEGAL, "ILLEGAL");
      assert_eq!(rtog::token::token::token_identifier::EOF, "EOF");
    }
  }
}
