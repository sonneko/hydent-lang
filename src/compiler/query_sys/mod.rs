use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::compiler::arena::ArenaBox;
use crate::diagnostic::{CompilerDiagnostic, diagnose_and_finish};

pub trait Query: 'static {
    type From: Hash + Clone;
    type To: Hash + Clone;
    fn run(db: &Database, src: Self::From) -> Self::To;
}

pub enum QuerySysFetchErr {
    DependentCycleDetected,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Revision(u32);
impl Revision {
    fn new() -> Self {
        Self(0)
    }
    fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Eq, PartialEq, std::hash::Hash, Copy, Clone)]
struct HashedQueryFrom(u64);
impl HashedQueryFrom {
    fn new<T: Hash>(t: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        HashedQueryFrom(hasher.finish())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct QueryId {
    query_type: TypeId,
    from: HashedQueryFrom,
}
impl QueryId {
    fn new<Q: Query>(hash: HashedQueryFrom) -> Self {
        Self {
            query_type: TypeId::of::<Q>(),
            from: hash,
        }
    }
}

pub struct Database {
    current_rivision: Revision,
    queries: HashMap<TypeId, Box<dyn Any>>,
    stack: Vec<QueryId>,
}
impl Database {
    fn fetch<Q: Query>(&mut self, from: Q::From) -> Result<Q::To, QuerySysFetchErr> {
        // 1. check there is no dependent cycle
        let current_query = QueryId::new::<Q>(HashedQueryFrom::new(&from));
        if self.stack.contains(&current_query) {
            return Err(QuerySysFetchErr::DependentCycleDetected);
        }

        // 2. add self to stack
        self.stack.push(current_query);

        // 3. save that parent query depends on this query
        if let Some(parent_query) = self.stack.last() {
            self.find(*parent_query).unwrap().dependencies.push(current_query);
        } else {
            // This query is root
        }

        // 4. check if there is a verified cached result
        let to = if let Some(slot) = self.find::<Q>(current_query) {
            if slot.verified_at == self.current_rivision {
                // verified to have runned in this rivision
                slot.to.clone()
            } else {
                // let's check if the cach can be used
                if self.check_query_is_up_to_date(current_query) {
                    slot.to.clone()
                } else {
                    Q::run(self, from)
                }
            }
        } else {
            // no cached result, run the query
            let to = Q::run(self, from);
            to
        };

        // 4. remove this query from stack
        self.stack.pop();

        Ok(to)
    }

    fn check_query_is_up_to_date(&self, query_id: QueryId) -> bool {
        todo!()
    }

    fn find<Q: Query>(&self, query_id: QueryId) -> Option<&Slot<Q>> {
        let found = self.queries.get(&TypeId::of::<Q>());
        let storage = found.map(|q| q.downcast_ref::<Storage<Q>>().unwrap())?;
        Some(storage.find(&query_id.from)?)
    }
}

pub struct Storage<Q: Query> {
    revision: Revision,
    data: HashMap<HashedQueryFrom, Slot<Q>>,
}
impl<Q: Query> Storage<Q> {
    fn find(&self, from: &HashedQueryFrom) -> Option<&Slot<Q>> {
        self.data.get(from)
    }
}

pub struct Slot<Q: Query> {
    to: Q::To,
    verified_at: Revision,
    changed_at: Revision,
    dependencies: Vec<QueryId>,
}
