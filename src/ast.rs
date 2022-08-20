#[derive(Clone, Debug)]
pub struct Expr {
    pub raw: String,
    pub span: Span,
    pub children: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    from: (usize, usize),
    to: (usize, usize),
}
