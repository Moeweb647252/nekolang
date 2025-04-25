pub mod cursor;
pub mod ptr;

pub fn is_left_bracket(c: char) -> bool {
  matches!(c, '{' | '[' | '(' | '<')
}

pub fn get_right_bracket_unwrap(c: char) -> char {
  match c {
    '{' => '}',
    '[' => ']',
    '(' => ')',
    '<' => '>',
    _ => unreachable!(),
  }
}

pub fn is_punctuation(c: char) -> bool {
  use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};
  c.general_category_group() == GeneralCategoryGroup::Punctuation
}
