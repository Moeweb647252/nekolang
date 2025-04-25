#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Modulus,
  Power,
  Assign,
  Equal,
}
