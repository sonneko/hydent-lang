use std::any::{Any, TypeId};
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::compiler::arena::ArenaBox;
use crate::diagnostic::{diagnose_and_finish, CompilerDiagnostic};

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

type Storage<T> = HashMap<TypeId, HashMap<HashedQueryFrom, T>>;

pub struct Database {
    current_rivision: Revision,
    queries_outputs: Storage<Box<dyn Any /* Query::To */>>,
    queries_metadata: Storage<QueryMetadata>,
    stack: Vec<QueryId>,
}

pub struct QueryMetadata {
    verified_at: Revision,
    changed_at: Revision,
    dependencies: Vec<QueryId>,
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
            self.find_query_metadata_mut(*parent_query)
                .unwrap()
                .dependencies
                .push(current_query);
        } else {
            // This query is root
        }

        // 4. check if there is a verified cached result
        let to = if self.find_query_output::<Q>(current_query).is_some() {
            let metadata = self.find_query_metadata(current_query).unwrap() as &QueryMetadata;
            if metadata.verified_at == self.current_rivision {
                // verified to have runned in this rivision
                self.find_query_output::<Q>(current_query).unwrap().clone()
            } else {
                // let's check if the cach can be used
                if self.check_query_is_up_to_date(current_query) {
                    self.find_query_output::<Q>(current_query).unwrap().clone()
                } else {
                    Q::run(self, from)
                }
            }
        } else {
            // no cached result, run the query
            Q::run(self, from)
        };

        // 4. remove this query from stack
        self.stack.pop();

        Ok(to)
    }

    fn check_query_is_up_to_date(&self, query_id: QueryId) -> bool {
        todo!()
    }

    fn find_query_output<Q: Query>(&self, query_id: QueryId) -> Option<&Q::To> {
        assert_eq!(query_id.query_type, TypeId::of::<Q>());
        let found = self.queries_outputs.get(&TypeId::of::<Q>())?;
        let storage = found.get(&query_id.from)?;
        Some(storage.downcast_ref::<Q::To>().unwrap())
    }

    fn find_query_metadata_mut(&mut self, query_id: QueryId) -> Option<&mut QueryMetadata> {
        let found = self.queries_metadata.get_mut(&query_id.query_type)?;
        found.get_mut(&query_id.from)
    }

    fn find_query_metadata(&self, query_id: QueryId) -> Option<&QueryMetadata> {
        let found = self.queries_metadata.get(&query_id.query_type)?;
        found.get(&query_id.from)
    }
}
