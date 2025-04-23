use crate::{keyword::Keyword, operator::Operator};

pub enum Token {
  Keyword(Keyword),
  Identifier(String),
  StringLiteral(String),
  NumberLiteral(String),
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
