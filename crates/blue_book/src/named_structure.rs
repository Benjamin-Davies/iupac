use std::fmt;

use crate::graph::Graph;

pub trait NamedStructure: fmt::Debug + Sync {
    fn to_graph(&self) -> Graph;

    fn as_any(&'static self) -> AnyNamedStructure
    where
        Self: Sized,
    {
        AnyNamedStructure { inner: self }
    }

    #[cfg(test)]
    fn to_ast(&'static self) -> std::rc::Rc<crate::parser::AST>
    where
        Self: Sized,
    {
        crate::parser::AST::Structure(self.as_any()).into()
    }
}

#[derive(Clone, Copy)]
pub struct AnyNamedStructure {
    inner: &'static dyn NamedStructure,
}

impl NamedStructure for AnyNamedStructure {
    fn to_graph(&self) -> Graph {
        self.inner.to_graph()
    }
}

impl fmt::Debug for AnyNamedStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl PartialEq for AnyNamedStructure {
    fn eq(&self, other: &Self) -> bool {
        // HACK
        format!("{self:?}") == format!("{other:?}")
    }
}

impl Eq for AnyNamedStructure {}
