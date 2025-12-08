## compiler fases

* SURFACE
    - `cli`: receive user's input and send that into `compiler` and return the results
    - `compiler`: manage all compile process and resource with scheduling
* FRONTEND(`CompilerFrontendContext`)
    - `tokenizer`: tokenize input and generate `Vec<Token>`
    - `parser`: parse `Vec<Token>` and generate `AST`
    - `dependency_resolution`: resolve file and package level devendency
    - `name_resolution`: resolve name and type
    - `type_checker`: check if type is correct
* MIDDLEEND(`CompilerMiddleendContext`)
    - `hir_gen`: generate hir that is more low-level and brief lang from `AST`
    - `hir_transform`: optimize or transform hir
    - `mir_gen`: generate mir from hir
    - `mir_transform`: optimize or transform mir
* BACKEND(`CompilerBackendContext`)
    - `llvmir_gen`: generate llvm-ir
    - `linker`: link the output binary to binaries from other programming language and export one binary
* OTHER
    - `diagnostic`: manage compiler diagnostic from any fases
    - `linter`: lint
    - `doc_gen`: generate documents from source code
    - `common`: utility for all fases



## build step

#### 整合性check@`tokenizer`,`parser`のチェック
- `/assets/grammer.bnf`の全非終端文字の定義の存在を確認する。
- `/assets/grammer.bnf`から再帰降下パーサの関数のシグネチャを生成し、`/spec/parser-functions.rs.txt'に保存する。
- 全関数が、`src/parser/parser.rs`に実装されていることを確認する。
- 非終端文字の一覧を、`/spec/non-terminal-characters.txt`に保存する。
- ASTの型定義にそれらが含まれていることを確認する。**<- TODO**
- 終端文字の一覧を、`/spec/terminal-characters.txt`に保存する。
- Tokenの型定義にそれらが含まれていることを確認する。**<- TODO**
- テストの実行 **<-TODO**
- 他のcheck **<-TODO**

#### 整合性check@
- 他のcheck **<-TODO**

#### エラーメッセージのビルド
- `cargo build`を実行する。
- 