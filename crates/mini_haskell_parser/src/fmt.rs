use crate::ast::{BinaryExpr, BinaryOp, CondExpr, Expr, ExprKind, Literal};
use debug_tree::scoped_branch::ScopedBranch;
use debug_tree::TreeBuilder;

pub struct AstViewBuilder {
    builder: TreeBuilder,
    root: ScopedBranch,
}

impl AstViewBuilder {
    pub fn new() -> Self {
        let builder = TreeBuilder::new();
        let root = builder.add_branch("AST");
        AstViewBuilder { builder, root }
    }

    pub fn node(&mut self, name: &str, node: &dyn AstView) -> &mut Self {
        let mut branch = self.builder.add_branch(name);
        node.view(self);
        branch.release();
        self
    }

    pub fn leaf(&mut self, value: &str) -> &mut Self {
        self.builder.add_leaf(value);
        self
    }

    pub fn string(&mut self) -> String {
        self.root.release();
        self.builder.string()
    }
}

pub trait AstView {
    fn view(&self, f: &mut AstViewBuilder);

    fn ast(&self) -> String {
        let mut builder = AstViewBuilder::new();
        self.view(&mut builder);
        builder.string()
    }
}

impl AstView for Expr {
    fn view(&self, f: &mut AstViewBuilder) {
        match &self.kind {
            ExprKind::Literal(lit) => lit.view(f),
            ExprKind::BinaryExpr(expr) => expr.view(f),
            ExprKind::CondExpr(expr) => expr.view(f),
        }
    }
}

impl AstView for Literal {
    fn view(&self, f: &mut AstViewBuilder) {
        match self {
            Literal::NatureNum(n) => {
                f.leaf(&format!("Nat({n})"));
            }
            Literal::Bool(b) => {
                f.leaf(&format!("Bool({b})"));
            }
        }
    }
}

impl AstView for BinaryExpr {
    fn view(&self, f: &mut AstViewBuilder) {
        f.node("lhs", &self.lhs)
            .leaf(&format!("ops: {}", self.op.raw()))
            .node("rhs", &self.rhs);
    }
}

impl AstView for CondExpr {
    fn view(&self, f: &mut AstViewBuilder) {
        f.node("condition", &self.condition)
            .node("then-branch", &self.then_branch)
            .node("else-branch", &self.else_branch);
    }
}

impl AstView for Box<Expr> {
    fn view(&self, f: &mut AstViewBuilder) {
        AstView::view(self.as_ref(), f);
    }
}

impl BinaryOp {
    fn raw(&self) -> &'static str {
        match self {
            BinaryOp::Plus => "+",
            BinaryOp::Ampersand => "&",
            BinaryOp::Less => "<",
            BinaryOp::Equal => "=",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fmt::AstView;
    use crate::parser::Parser;
    use testsuite::unittest;

    unittest!(expr, |_, src| {
        let ast = Parser::new(&src).conditional().unwrap().ast();
        insta::assert_snapshot!(ast);
    });
}
