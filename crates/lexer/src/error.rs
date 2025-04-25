use thiserror::Error;
use utils::cursor::Pos;

#[derive(Error, Debug)]
pub enum LexError {
  #[error("Unexpected character: {0}, at position: {1}")]
  UnexpectedCharacter(char, Pos),
  #[error("Unexpected end of input, at position: {0}")]
  UnexpectedEndOfInput(Pos),
  #[error("Missing closing bracket: {0}, at position: {1}")]
  MissingClosingBracket(char, Pos),
  #[error("Eof")]
  Eof,
  #[error("Unexpected punctuation: {0}, at position: {1}")]
  UnexpectedPunctuation(char, Pos),
}
