### Hydent言語リファレンス (Hydent Language Reference)

#### 1. 概要 (Introduction)

Hydentは、安全性、パフォーマンス、そして優れた開発者体験の両立を目指して設計された、静的型付きインタプリタ/コンパイラ言語です。LLVMバックエンドによるJITコンパイルorAOTコンパイルを実行します。

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

Hydentは式ベースの言語です。多くの構文が値を返します。

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

### 9. 構文のBNF (BNF Grammar for Hydent)

```bnf
<ast> ::= <top_level>

<top_level> ::= { <top_level_statement> }

<top_level_statement> ::=
        <import_declaration>
      | <static_variable_declaration>
      | <class_declaration>
      | <enum_declaration>
      | <struct_declaration>
      | <function_declaration>
      | <protocol_declaration>
      | <module_declaration>
      | <annotation>
      | <type_alias_declaration>

<import_declaration> ::= "import" ( <import_specific> | <import_all_as> ) "from" <STRING_LITERAL> ";"
<import_specific> ::= "{" <identifier_list> "}"
<import_all_as> ::= "*" "as" <IDENTIFIER>

<static_variable_declaration> ::= <docs_comments> <is_public> "static" <IDENTIFIER> "=" <expression> ";"

<class_declaration> ::= <docs_comments> <is_public> "class" <IDENTIFIER> <generics> <implements_protocol> "{" { <function_declaration> | <field_declaration> | <type_alias_declaration> } "}"

<enum_declaration> ::= <docs_comments> <is_public> "enum" <IDENTIFIER> <generics> <implements_protocol> "{" { <enum_member> } "}"
<enum_member> ::= <enum_variant> | <function_declaration>
<enum_variant> ::= <IDENTIFIER> ( "(" <type_literal_list> ")" )?

<struct_declaration> ::= <docs_comments> <is_public> "struct" <IDENTIFIER> <struct_body>
<struct_body> ::= <struct_block_body> | <struct_tuple_body> | ";"
<struct_block_body> ::= "{" { <field_declaration> } "}"
<struct_tuple_body> ::= "(" <type_literal_list> ")"

<function_declaration> ::= <docs_comments> "extern"? <is_public> "async"? "fn" <IDENTIFIER> <generics> "(" <params_with_types>  ")" ( "->" <type_literal> )? "panics"? <block_expression>?

<protocol_declaration> ::= <docs_comments> <is_public> "protocol" <IDENTIFIER> <implements_protocol> "{" { <protocol_member> } "}"
<protocol_member> ::= <function_declaration> | <type_alias_declaration>

<module_declaration> ::= <docs_comments> <is_public> "module" <IDENTIFIER> "{" <top_level> "}"

<annotation> ::= "@" <IDENTIFIER> { <literal> }

<type_alias_declaration> ::= <docs_comments> <is_public> "type" <IDENTIFIER> <generics>? "=" <type_literal> ";"

<if_expression> ::= "if" <expression> <block_expression> { <else_if_clause> } <else_clause>?
<else_if_clause> ::= "else" "if" <expression> <block_expression>
<else_clause> ::= "else" <block_expression>

<match_expression> ::= "match" <expression> "{" { <match_arm> } "}"
<match_arm> ::= <pattern> ( "if" <expression> )? "=>" <expression>

<loop_expression> ::= "loop" <expression>? <block_expression>

<while_expression> ::= "while" <expression> <block_expression>

<for_statement> ::= "for" <pattern> "in" <expression> <block_expression>

<for_expression> ::= "for" <expression> "{" { <pipeline_arm> } "}"
<pipeline_arm> ::= "|>" <pattern> "=>" <expression>

<if_let_expression> ::= "if" "let" <pattern> "=" <expression> <block_expression>
<while_let_expression> ::= "while" "let" <pattern> "=" <expression> <block_expression>

<pipe_expression> ::= "pipe" <expression> "{" { <pipe_arm> } "}"
<pipe_arm> ::= "|>" <pattern> ("if" <expression>)? "=>" <expression>

<closer> ::= "(" <closer_params>? ")" "->" <expression>
<closer_params> ::= <closer_param_item> { "," <closer_param_item> }
<closer_param_item> ::= <param_with_type> | <IDENTIFIER>

<accesser> ::= <IDENTIFIER> { "::" <IDENTIFIER> }

<params> ::= <expression_list>?

<expression> ::= <logical_or_expr>
<logical_or_expr> ::= <logical_and_expr> { "||" <logical_and_expr> }
<logical_and_expr> ::= <bitwise_or_expr> { "&&" <bitwise_or_expr> }
<bitwise_or_expr> ::= <bitwise_xor_expr> { "|" <bitwise_xor_expr> }
<bitwise_xor_expr> ::= <bitwise_and_expr> { "^" <bitwise_and_expr> }
<bitwise_and_expr> ::= <equality_expr> { "&" <equality_expr> }
<equality_expr> ::= <relational_expr> { ("==" | "!=") <relational_expr> }
<relational_expr> ::= <shift_expr> { ("<" | "<=" | ">" | ">=") <shift_expr> }
<shift_expr> ::= <additive_expr> { ("<<" | ">>") <additive_expr> }
<additive_expr> ::= <multiplicative_expr> { ("+" | "-") <multiplicative_expr> }
<multiplicative_expr> ::= <power_expr> { ("*" | "/" | "%") <power_expr> }
<power_expr> ::= <prefix_expr> { "**" <prefix_expr> }
<prefix_expr> ::= (("!" | "~" | "-") <prefix_expr>) | <primary_expr>
<primary_expr> ::=
      <block_expression>
    | <if_expression>
    | <match_expression>
    | <loop_expression>
    | <while_expression>
    | <for_expression>
    | <pipe_expression>
    | <accesser>
    | <literal>
    | <function_call>
    | <method_call>
    | <field_access>
    | "await" <expression>
    | <tuple_or_grouped_expression>
    | <struct_literal>
    | <closer>
    | <if_let_expression>
    | <while_let_expression>
    | <array_literal>
    | <index_access>
    | <cast_expression>

<function_call> ::= "try"? <accesser> "(" <params> ")"
<method_call> ::= "try"? <accesser> "." <IDENTIFIER> "(" <params> ")"
<field_access> ::= <accesser> "." <IDENTIFIER>
<tuple_or_grouped_expression> ::= "(" <expression_list>? ")"
<struct_literal> ::= <accesser> "{" <struct_literal_fields>? "}"
<struct_literal_fields> ::= <struct_field_init> { "," <struct_field_init> }
<struct_field_init> ::= ( <IDENTIFIER> ":" <expression> ) | <IDENTIFIER>
<array_literal> ::= "[" <expression_list>? "]"
<index_access> ::= <expression> "[" <expression> "]"
<cast_expression> ::= <expression> "as" <type_literal>


<statement> ::=
      <if_expression>
    | <match_expression>
    | <loop_expression>
    | <while_expression>
    | <for_statement>
    | <expression_statement>
    | <variable_declaration>
    | "return" <expression> ";"
    | "break" <expression> ";"
    | "continue" ";"
    | <assignment_statement>

<expression_statement> ::= "ignore"? <expression> ";"
<variable_declaration> ::= <variable_declaration_keyword> <pattern> ( ":" <type_literal> )? <variable_declaration_assignment>? ";"
<variable_declaration_keyword> ::= "let" | "const"
<variable_declaration_assignment> ::= <ASSIGNMENT_OPERATOR> <expression>
<assignment_statement> ::= <accesser> <ASSIGNMENT_OPERATOR> <expression> ";"

<field_declaration> ::= ( "final" | "mut" ) <IDENTIFIER> ":" <type_literal> ";"

<param_with_type> ::= "mut"? ( <IDENTIFIER> ":" <type_literal> ( "=" <expression> )? ) | "this"
<params_with_types> ::= <param_with_types_list>?
<param_with_types_list> ::= <param_with_type> { "," <param_with_type> }

<block_expression> ::= "{" { <statement> } | <expression> "}"

<is_public> ::= "pub" | "";

<literal> ::= <STRING_LITERAL> | <CHAR_LITERAL> | <NUM_LITERAL> | <BOOL_LITERAL>

<type_literal> ::=
      <accesser> <generic_type_args>?
    | "impl" <type_literal>
    | "typeof" <expression>
    | "Bool"
    | "Int"
    | "DoubleInt"
    | "Float"
    | "DoubleFloat"
    | "Char"
    | "Usize"
    | "Any"
    | <tuple_type>
    | "Never"
    | "Void"

<generic_type_args> ::= "<" <type_literal_list> ">"
<tuple_type> ::= "(" <type_literal_list>? ")"

<docs_comments> ::= { <DOCS_COMMENT> } { <annotation> }

<pattern> ::=
      <IDENTIFIER>
    | "_"
    | <tuple_struct_pattern>
    | <tuple_pattern>
    | <struct_pattern>
    | <accesser>
    | <literal>
    | <range_pattern>
    | <binding_pattern>
# TODO: add [ first, second, ... last ] like pattern

<tuple_struct_pattern> ::= <accesser> "(" <pattern_list>? ")"
<tuple_pattern> ::= "(" <pattern_list>? ")"
<struct_pattern> ::= <accesser> "{" <struct_pattern_fields>? "}"
<struct_pattern_fields> ::= <struct_pattern_field> { "," <struct_pattern_field> }
<struct_pattern_field> ::= ( <IDENTIFIER> ":" <pattern> ) | <IDENTIFIER>
<range_pattern> ::= ( <CHAR_LITERAL> | <NUM_LITERAL> ) <range_op> ( <CHAR_LITERAL> | <NUM_LITERAL> )
<range_op> ::= ".." | "..="
<binding_pattern> ::= <IDENTIFIER> "@" <pattern>

<generics> ::= "<" <generic_param_def_list> ">"
<generic_param_def_list> ::= <generic_param_def> { "," <generic_param_def> }
<generic_param_def> ::= <IDENTIFIER> ( ":" <generic_bound> )?
<generic_bound> ::= <type_literal> { "&" <type_literal> }

<implements_protocol> ::= ( ":" <accesser_list> )?

# Common List Rules
<identifier_list> ::= <IDENTIFIER> { "," <IDENTIFIER> }
<type_literal_list> ::= <type_literal> { "," <type_literal> }
<expression_list> ::= <expression> { "," <expression> }
<pattern_list> ::= <pattern> { "," <pattern> }
<accesser_list> ::= <accesser> { "," <accesser> }

# <IDENTIFIER>: identifier like es_2_d22
# <STRING_LITERAL>: string literal like "HelloWor\n\"ld"
# <CHAR_LITERAL>: char literal like '*' '\n' '\''
# <NUM_LITERAL>: number literal like 243 42.234 0xff 0b00101001 423.323e+2
# <BOOL_LITERAL>: true or false
# <DOCS_COMMENT>: documentation comment like /// # This is title. \n it's content.
# <ASSIGNMENT_OPERATOR>: = -= += *= /= %= **=

```