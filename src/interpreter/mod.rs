use crate::ast::*;
use crate::val::Val;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use color_eyre::eyre;

pub mod base;

/// A tree representing all of the different scopes of the program.
pub struct Scope {
    key: String,
    children: HashSet<Scope>,
    vals: HashMap<String, Val>,
}
impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
impl Scope {
    pub fn global() -> Self {
        let mut vals = HashMap::new();
        Self {
            key: "GLOBAL".to_string(),
            children: HashSet::new(),
            vals,
        }
    }
}


/// Interprets a list of expression trees in the global scope.
pub fn interpret(expressions: &[Expr]) -> eyre::Result<()> {
    let mut scope = Scope::global();

    for expr in expressions {
        eval(expr, &mut scope);
    }

    Ok(())
}

/// Evaluates an expression, altering the scope if needed.
pub fn eval(_expr: &Expr, _scope: &mut Scope) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(interpret(&[]).is_ok());
    }
}
