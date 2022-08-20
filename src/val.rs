#[derive(Clone, Debug)]
pub enum Val {
    Int(i64),
    Float(f64),
    Str(String),
    Func {
        params: Vec<String>,
        body: Vec<crate::ast::Expr>,
    }
}
