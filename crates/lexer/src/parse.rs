use crate::{Lexer, error::LexError};
use reference::{keyword::Keyword, token::Token};
use utils::cursor::Cursor;

impl Lexer {
  pub fn new(input: impl ToString) -> Self {
    Self {
      input: input.to_string(),
    }
  }

  pub fn parse(&self) -> Result<Vec<Token>, LexError> {
    let mut cursor = Cursor::new(self.input.clone());
    let mut met = false;
    parse_scope(&mut cursor, '\0', &mut met)
  }
}

fn parse_scope(cur: &mut Cursor, stop_at: char, met: &mut bool) -> Result<Vec<Token>, LexError> {
  let mut tokens = Vec::new();
  while let Some(c) = cur.peek() {
    match c {
      x if x.is_whitespace() => {
        cur.skip_whitespace();
      }
      x if x == stop_at => {
        *met = true;
        cur.forward();
        break;
      }
      x if utils::is_left_bracket(x) => {
        let stop_at = utils::get_right_bracket_unwrap(x);
        cur.forward();
        let mut met = false;
        let scope_tokens = parse_scope(cur, stop_at, &mut met)?;
        if !met {
          return Err(LexError::MissingClosingBracket(stop_at, cur.pos()));
        }
        tokens.push(match x {
          '{' => Token::Brace(scope_tokens),
          '[' => Token::Bracket(scope_tokens),
          '(' => Token::Parenthesis(scope_tokens),
          '<' => Token::AngleBracket(scope_tokens),
          _ => unreachable!(),
        });
      }
      '"' => {
        tokens.push(parse_string(cur)?);
      }
      _ => {
        tokens.push(match parse_token(cur)? {
          Some(token) => token,
          None => break,
        });
      }
    }
  }
  Ok(tokens)
}

fn parse_token(cur: &mut Cursor) -> Result<Option<Token>, LexError> {
  let token = match cur.peek() {
    Some(c) if c.is_ascii_digit() => Some(parse_number(cur)?),
    Some(c) if utils::is_punctuation(c) => Some(parse_punctuation(cur)?),
    Some(_) => parse_word(cur)?,
    None => return Ok(None),
  };
  Ok(token)
}

fn parse_word(cur: &mut Cursor) -> Result<Option<Token>, LexError> {
  let mut word = String::new();
  while let Some(c) = cur.peek() {
    if c.is_whitespace() || utils::is_punctuation(c) {
      break;
    }
    word.push(c);
    cur.forward();
  }
  if word.is_empty() {
    return Ok(None);
  }
  let token = match word.as_str() {
    "true" => Token::BooleanLiteral(true),
    "false" => Token::BooleanLiteral(false),
    "if" => Token::Keyword(Keyword::If),
    "else" => Token::Keyword(Keyword::Else),
    "while" => Token::Keyword(Keyword::While),
    "for" => Token::Keyword(Keyword::For),
    "return" => Token::Keyword(Keyword::Return),
    "break" => Token::Keyword(Keyword::Break),
    "continue" => Token::Keyword(Keyword::Continue),
    "fn" => Token::Keyword(Keyword::Fn),
    "let" => Token::Keyword(Keyword::Let),
    "const" => Token::Keyword(Keyword::Const),
    "gen" => Token::Keyword(Keyword::Gen),
    "loop" => Token::Keyword(Keyword::Loop),
    "match" => Token::Keyword(Keyword::Match),
    "use" => Token::Keyword(Keyword::Use),
    "in" => Token::Keyword(Keyword::In),
    "and" => Token::Keyword(Keyword::And),
    "or" => Token::Keyword(Keyword::Or),
    "not" => Token::Keyword(Keyword::Not),
    _ => Token::Identifier(word),
  };
  Ok(Some(token))
}

fn parse_number(cur: &mut Cursor) -> Result<Token, LexError> {
  let mut number = String::new();
  for c in cur.by_ref() {
    if c.is_ascii_digit() || c == '.' || c == '_' {
      number.push(c);
    } else {
      break;
    }
  }
  Ok(Token::NumberLiteral(number))
}

fn parse_punctuation(cur: &mut Cursor) -> Result<Token, LexError> {
  let c = cur.peek().unwrap();
  cur.forward();
  let token = match c {
    ';' => Token::Semicolon,
    ',' => Token::Comma,
    '.' => Token::Dot,
    ':' => Token::Colon,
    _ => return Err(LexError::UnexpectedPunctuation(c, cur.pos())),
  };
  Ok(token)
}

fn parse_string(cur: &mut Cursor) -> Result<Token, LexError> {
  let mut string = String::new();
  cur.forward();
  while let Some(c) = cur.peek() {
    if c == '"' {
      cur.forward();
      break;
    }
    string.push(c);
    cur.forward();
  }
  Ok(Token::StringLiteral(string))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Lexer;
  use reference::{operator::Operator, token::Token};

  #[test]
  fn test_parse() {
    let input = "
      let x = 10;
      let y = 20;
      if x > y {
        println(\"x is greater than y\");
      } else {
        println(\"y is greater than x\");
      }
    ";
    let lexer = Lexer::new(input);
    let tokens = lexer.parse().unwrap();
    println!("{:?}", tokens);
  }
}
