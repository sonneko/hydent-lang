pub mod query_sys;

use std::hash::Hash;

pub enum RuntimeErr {}

pub trait Engine {
    fn fetch<Q: Query>(&self, from: Q::From) -> Result<Q::To, RuntimeErr>;
}

pub struct Rumtime<E: Engine> {
    engine: E,
}

pub trait Query: 'static {
    type From: Hash + Clone;
    type To: Hash + Clone;
    fn run<E: Engine>(engine: &E, src: Self::From) -> Self::To;
}
