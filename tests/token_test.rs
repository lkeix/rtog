extern crate rstest;
#[cfg(test)]
extern crate speculate;

use rstest::*;
use speculate::speculate;
use token::token_identifier::*;

speculate! {
  describe "Check constant tokens"  {
    fn test_constant_token() {
      assert_eq!(ILLEGAL, "")
    }
  }
}
