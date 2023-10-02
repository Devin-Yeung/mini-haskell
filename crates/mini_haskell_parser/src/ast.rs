use mini_haskell_diagnostic::span::Span;

#[derive(Debug)]
pub struct Expr {
    kind: ExprKind,
    span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Literal),
    BinaryExpr(BinaryExpr),
    CondExpr(CondExpr),
}

#[derive(Debug)]
pub enum Literal {
    NatureNum(usize),
    Bool(bool),
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Ampersand,
    Less,
    Equal,
}

#[derive(Debug)]
pub struct BinaryExpr {
    lhs: Box<Expr>,
    op: BinaryOp,
    rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct CondExpr {
    condition: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}
