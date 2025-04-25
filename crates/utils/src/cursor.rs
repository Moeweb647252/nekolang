use std::fmt::Display;

#[derive(Debug)]
pub struct Cursor {
  data: Vec<char>,
  current: usize,
  pos: Pos,
}

impl Cursor {
  pub fn new(data: impl ToString) -> Self {
    let data = data.to_string().chars().collect();
    Self {
      data,
      current: 0,
      pos: Pos { line: 0, column: 0 },
    }
  }

  pub fn back(&mut self) -> Option<char> {
    if self.current == 0 {
      return None;
    }
    self.current -= 1;
    self.data.get(self.current).copied()
  }

  pub fn peek(&self) -> Option<char> {
    self.data.get(self.current).copied()
  }

  pub fn get_pos(&self) -> usize {
    self.current
  }

  pub fn set_pos(&mut self, pos: usize) -> Result<(), String> {
    if pos > self.data.len() {
      return Err("Out of bounds".to_string());
    }
    self.current = pos;
    Ok(())
  }

  pub fn forward(&mut self) {
    self.current += 1;
    self.pos.column += 1;
  }

  pub fn pos(&self) -> Pos {
    self.pos.clone()
  }

  pub fn skip_whitespace(&mut self) {
    while let Some(c) = self.peek() {
      if c == '\n' {
        self.current += 1;
        self.pos.line += 1;
        self.pos.column = 0;
      } else if c.is_whitespace() {
        self.current += 1;
        self.pos.column += 1;
      } else {
        break;
      }
    }
  }
}

impl Iterator for Cursor {
  type Item = char;
  fn next(&mut self) -> Option<Self::Item> {
    self.forward();
    self.data.get(self.current).copied()
  }
}

#[derive(Debug, Clone)]
pub struct Pos {
  pub line: usize,
  pub column: usize,
}

impl Display for Pos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "line: {}, col: {}", self.line, self.column)
  }
}
