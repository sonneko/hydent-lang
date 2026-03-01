use crate::compiler::{
    arena::{ArenaBox, ArenaIter},
    span::Span,
    symbol::Symbol,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Module {
    imports: ArenaIter<Import>,
    static_variables: ArenaIter<StaticVariable>,
    classes: ArenaIter<Class>,
    modules: ArenaIter<Module>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Import {
    items: ArenaIter<Symbol>,
    path: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct StaticVariable {
    name: Symbol,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Class {
    name: Symbol,
    implements_protocol: ArenaIter<Symbol>,
}
