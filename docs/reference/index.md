### Aya言語リファレンス (Aya Language Reference)

#### 1. 概要 (Introduction)

Ayaは、安全性、パフォーマンス、そして優れた開発者体験の両立を目指して設計された、静的型付きインタプリタ/コンパイラ言語です。LLVMバックエンドによるJITコンパイルを実行します。

*   **設計哲学**:
    *   **構文 (Syntax)**: TypeScriptに類似した、親しみやすく直感的な構文を採用。
    *   **思想 (Philosophy)**: Rustの安全性（Null安全、厳格なエラーハンドリング）と表現力を参考にしつつ、所有権モデルは含まず、アプリケーション開発に最適化。
    *   **基盤 (Foundation)**: Javaの「全てはオブジェクト」という思想に基づき、一貫したオブジェクトモデルを提供。

#### 2. 字句構造 (Lexical Structure)

*   **コメント**:
    *   `//`: 行末までをコメントアウトします。コンパイラから完全に無視されます。
    *   `///`: ドキュメンテーションコメント。Markdown形式で記述され、開発者のための自由な説明を提供します。コンパイラからは無視されますが、ドキュメント生成ツールによって解釈されます。

*   **キーワード**:
    `fn`, `class`, `protocol`, `enum`, `impl`, `let`, `const`, `mut`, `final`, `if`, `else`, `match`, `while`, `for`, `in`, `return`, `pub`, `static`, `namespace`, `import`, `from`, `try`, `panic`, `self`, `Self`. プリミティブ型 (`Int`, `Float`, `Bool`, `Char`) およびリテラル (`true`, `false`) も予約されています。

*   **識別子と命名規則**:
    *   型名 (クラス, プロトコル, enum): `UpperCamelCase`
    *   変数名、関数名: `snake_case`
    *   静的関連定数: `UPPER_SNAKE_CASE`

#### 3. 型システム (Type System)

*   **型付け**: **名目的型付け (Nominal Typing)** を採用。構造が同じでも宣言が異なれば、別の型として扱われます。
*   **プリミティブ型**:
    *   `Int`: 32ビット符号付き整数。
    *   `DoubleInt`: 64ビット符号付き整数。
    *   `Float`: 32ビット浮動小数点数。
    *   `DoubleFloat`: 64ビット浮動小数点数。
    *   `Bool`: `true` または `false`。
    *   `Char`: Unicodeスカラ値。
*   **複合型**:
    *   `class`: データと振る舞いをカプセル化します。
    *   `enum`: バリアントを持つ代数的データ型を定義します。
*   **抽象化**:
    *   `protocol`: 特定の振る舞いの規約を定義します。クラスはプロトコルを実装できます。
    *   **ジェネリクス**: 型パラメータ化されたクラス、関数、プロトコルを定義できます。例: `class List<T> { ... }`
*   **Null安全**: `Option<T>` enum (`Some(T)` | `None`) を標準ライブラリで提供し、nullポインタの存在をコンパイル時に根絶します。
*   **エラーハンドリング型**: `Result<T, E>` enum (`Ok(T)` | `Err(E)`) を標準ライブラリで提供し、回復可能なエラーを表現します。

#### 4. 宣言 (Declarations)

*   **変数宣言**:
    *   `let name = value;`: 不変な参照を持つ変数を宣言します。インスタンスのフィールドはデフォルトで可変です。
    *   `const name = value;`: 深く不変な定数を宣言します。
    *   `static NAME = value;`: 深く不変な定数を宣言します。コンパイル時に確定する必要があります。
    *   `final` (フィールド修飾子): クラスのフィールドを不変にします。

*   **関数宣言**:
    *   `fn name(param: Type) -> ReturnType { ... }`
    *   メソッドの第一引数には `self` または `mut self` を明示します。`self` を取らないメソッドは静的な関連関数です。

*   **クラス宣言**:
    *   `class Name: Protocol { final field: Type; ... }`
    *   コンストラクタは特別な構文を持たず、`::new()` という名前の静的関連関数として実装することが推奨されます。

*   **アクセス制御**:
    *   `pub`: アイテム（関数、クラス、フィールド等）をモジュール外に公開します。
    *   デフォルトは非公開（モジュールプライベート）です。

*   **静的関連定数**:
    *   `class MyClass { static MY_CONST: Int = 10; }`

#### 5. 式 (Expressions)

Ayaは式ベースの言語です。多くの構文が値を返します。

*   **制御フロー式**:
    *   `if condition { ... } else { ... }`: `if` は式であり、各ブロックの最後の式がその値を返します。
    *   `if let Some(x) = my_option { ... }`
    *   `match value { pattern => result, ... }`
*   **ブロック式**: ` { ... } ` で囲まれたブロックは、その中の最後の式を値として返します。
*   **戻り値**:
    *   ブロックが単一の式で構成される場合、その式の結果が自動的にブロックの戻り値となります。
    *   ブロック内に複数の文がある場合、値を返すには明示的な `return` が必要です。
*   **アクセス演算子**:
    *   `.`: インスタンスの動的なメンバー（フィールド、メソッド）にアクセスします。
    *   `::`: 型の静的なメンバー（関連関数、関連定数）にアクセスします。

#### 6. エラーハンドリング (Error Handling)

*   **回復可能なエラー**: `Result<T, E>` 型と `?` 演算子を使用します。
    *   `let value = might_fail()?;` は `might_fail()` が `Ok(v)` なら `v` を、`Err(e)` なら現在の関数から `Err(e)` を返します。
*   **回復不能なエラー (パニック)**:
    *   `panic("message")`: プログラムの実行を即座に停止させます。これはバグや事前条件の違反など、回復不能な状況でのみ使用されます。
    *   `try fn`: 関数が `panic` を呼び出す可能性があることを示すマーカーです。`try fn` を呼び出す関数も `try fn` としてマークされる必要があります。これにより、パニックの危険性が静的に追跡されます。

#### 7. メタプログラミングとドキュメンテーション

*   **`@属性 (Attribute)`**: `@name(...)`
    *   コンパイラにコードのコンパイル方法を指示する命令。プログラムのセマンティクスに直接影響します。（例: `@inline`, `@deprecated(...)`）
*   **`#ディレクティブ (Directive)`**: `#name ...`
    *   関数やクラスの「契約」を記述するための構造化されたメタデータ。セマンティックルール（リンター）のチェック対象となり、ドキュメントを生成します。（例: `#summary "..."`, `#params name: "..."`, `#panics "..."`）
*   **`///ドキュメンテーションコメント`**:
    *   人間が読むための自由記述のMarkdownコメント。コンパイラからは完全に無視されます。

---

### 9. 構文のBNF (BNF Grammar for Aya)

```bnf
<program> ::= { <top_level_item> }

<top_level_item> ::= <import_statement> | <namespace_block> | <declaration>

<import_statement> ::= "import" ( <identifier> | "{" <comma_separated_identifiers> "}" ) "from" <string_literal> ";"

<declaration> ::= <function_decl> | <class_decl> | <protocol_decl> | <enum_decl> | <static_decl>

<visibility> ::= "pub" | ""

(* ---- Declarations ---- *)

<function_decl> ::= { <directive> } { <attribute> } <visibility> [ "try" ] "fn" <identifier> [ <generic_params> ] "(" [ <param_list> ] ")" [ "->" <type> ] <block_expr>

<param_list> ::= <param> { "," <param> }
<param> ::= <identifier> ":" <type>

<class_decl> ::= { <directive> } { <attribute> } <visibility> "class" <identifier> [ <generic_params> ] [ ":" <type> ] "{" { <class_member> } "}"

<class_member> ::= <field_decl> | <function_decl> | <static_const_decl>

<field_decl> ::= { <directive> } <visibility> [ "final" ] <identifier> ":" <type> ";"

<static_const_decl> ::= "static" <const_decl> ";"

<const_decl> ::= "const" <identifier> [ ":" <type> ] "=" <expression>

<static_decl> ::= "static" <identifier> [ ":" <type> ] "=" <expression>

<protocol_decl> ::= { <directive> } <visibility> "protocol" <identifier> [ <generic_params> ] "{" { <function_signature> } "}"
<function_signature> ::= <visibility> "fn" <identifier> "(" [ <param_list> ] ")" [ "->" <type> ] ";"

(* ---- Types ---- *)

<type> ::= <type_identifier> [ "<" <type_list> ">" ]
<type_list> ::= <type> { "," <type> }

(* ---- Statements ---- *)
(* Note: Aya is expression-oriented, so statements are a subset of expressions or declarations *)

<statement> ::= <declaration> | <let_statement> | <expression_statement>
<let_statement> ::= "let" [ "mut" ] <identifier> [ ":" <type> ] "=" <expression> ";"
<expression_statement> ::= <expression> ";"

(* ---- Expressions ---- *)

<expression> ::= <assignment_expr>

<assignment_expr> ::= <logical_or_expr> [ "=" <assignment_expr> ]

<logical_or_expr> ::= <logical_and_expr> { "||" <logical_and_expr> }
<logical_and_expr> ::= <equality_expr> { "&&" <equality_expr> }
<equality_expr> ::= <comparison_expr> { ( "==" | "!=" ) <comparison_expr> }
<comparison_expr> ::= <term_expr> { ( "<" | ">" | "<=" | ">=" ) <term_expr> }
<term_expr> ::= <factor_expr> { ( "+" | "-" ) <factor_expr> }
<factor_expr> ::= <unary_expr> { ( "*" | "/" ) <unary_expr> }

<unary_expr> ::= ( "!" | "-" ) <unary_expr> | <postfix_expr>

<postfix_expr> ::= <primary_expr> { <postfix_op> }
<postfix_op> ::= "?" | <call_op> | <member_access_op>
<call_op> ::= "(" [ <argument_list> ] ")"
<member_access_op> ::= "." <identifier>

<primary_expr> ::= <literal>
                 | <identifier>
                 | "(" <expression> ")"
                 | <if_expr>
                 | <match_expr>
                 | <block_expr>
                 | <return_expr>

(* ---- Control Flow and Block Expressions ---- *)

<if_expr> ::= "if" <expression> <block_expr> [ "else" ( <if_expr> | <block_expr> ) ]

<match_expr> ::= "match" <expression> "{" { <match_arm> } "}"
<match_arm> ::= <pattern> "=>" <expression> ","

<block_expr> ::= "{" { <statement> } [ <expression> ] "}"

<return_expr> ::= "return" [ <expression> ]

(* ---- Literals and Identifiers ---- *)

<literal> ::= <integer_literal> | <float_literal> | <bool_literal> | <char_literal> | <string_literal> | "None"
<identifier> ::= /* A sequence of letters, digits, and underscores, not starting with a digit */

(* ---- Directives and Attributes ---- *)

<directive> ::= "#" <identifier> <string_literal>
<attribute> ::= "@" <identifier> [ "(" <literal_list> ")" ]

```