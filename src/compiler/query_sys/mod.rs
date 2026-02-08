use std::collections::HashMap;
use std::hash::Hash;

use crate::diagnostic::CompilerDiagnostic;

pub trait Query {
    type From: Hash + Clone;
    type To: Hash + Clone;
    fn run(db: &Database, src: Self::From) -> Result<Self::To, Box<dyn CompilerDiagnostic>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Revision(u32);

pub struct DependenciesLoopErr;

// どのクエリのどのインスタンスかを指し示す識別子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryDescriptor {
    query_type_id: std::any::TypeId,
    input_hash: u64,
}

/// クエリの実行結果と依存関係を保持するスロット
pub struct Slot<Q: Query> {
    // 最後にこのクエリが実行、または検証されたリビジョン
    verified_at: Revision,
    // 最後にこのクエリの結果（ハッシュ）が実際に変化したリビジョン
    changed_at: Revision,
    // 実行結果のキャッシュ
    memo: Option<Memo<Q::To>>,
}

pub struct Memo<T> {
    value: T,
    value_hash: u64,
    dependencies: Vec<QueryDescriptor>,
}

pub struct Database {
    current_revision: Revision,
    // 関数を呼んだら一つ追加・関数が終わったら一つ取り除く
    stack: Vec<QueryDescriptor>,
    storage: QueryStorage,
}

pub struct QueryStorage {
    /* クエリの種類(Queryトレイトを実装する構造体の種類)それぞれを保存するための場所。*/
    // compile: Strorage<CompileQuery>,
    // read_file: Storage<ReadFileQuery>,
    // parse_file: Storage<ParseFileQuery>,
    // typecheck_function: Storage<TypecheckFunctionQuery>,
    // typecheck_file: Storage<TypecheckFileQuery>,
    /* ... */
}

pub struct Strage<Q: Query> {
    map: HashMap<Q::From, Slot<Q>>,
}
