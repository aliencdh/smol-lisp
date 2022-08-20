//! A collection of base functions, implemented directly in Rust.

use crate::val::Val;
use crate::interpreter::Scope;
use color_eyre::eyre;

impl Scope {
    /// Defines a variable.
    pub fn define(&mut self, name: String, val: Val) -> eyre::Result<()> {
        if self.vals.contains_key(&name) {
            eyre::bail!("Cannot rebind `{name}`.");
        }
        self.vals.insert(name, val);
        Ok(())
    }
}

