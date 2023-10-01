#[derive(Debug)]
pub enum Expr {
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
