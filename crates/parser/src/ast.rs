use utils::ptr::P;

pub struct File {
  filename: String,
}

pub struct Path {}

pub struct Block {}

pub struct Stmt {}

pub struct Expr {}

pub enum ExprKind {}

pub struct Type {
  kind: TypeKind,
}

pub enum TypeKind {
  Path(P<Path>),
  Tuple(Vec<P<Type>>),
  Vec(P<Type>),
}

pub struct Func {}

pub struct Closure {}

pub struct Struct {}

pub struct Call {}

pub struct MethodCall {}
