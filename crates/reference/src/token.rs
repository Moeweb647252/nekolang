use crate::{keyword::Keyword, operator::Operator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  Keyword(Keyword),
  Identifier(String),
  StringLiteral(String),
  NumberLiteral(String),
  BooleanLiteral(bool),
  Brace(Vec<Token>),
  Parenthesis(Vec<Token>),
  Bracket(Vec<Token>),
  AngleBracket(Vec<Token>),
  Operator(Operator),
  Semicolon,
  Comma,
  Dot,
  Colon,
}
