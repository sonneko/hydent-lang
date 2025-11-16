# Aya プログラミング言語

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/Aya-lang)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/Aya-lang)
[![Version](https://img.shields.io/badge/version-0.1.0_(Conceptual)-informational)](https://github.com/Aya-lang)

**Aya**は、安全性、ハイパフォーマンス、そして保守性の高いアプリケーションを構築するために設計された、モダンな静的型付けJITコンパイル言語です。TypeScriptに触発された親しみやすく直感的な構文と、Rustから得られた堅牢な設計哲学を組み合わせています。

## 基本哲学

Ayaは、4つの基本理念の上に成り立っています。

1.  **実用的な安全性 (Pragmatic Safety)**: 実行時エラーのクラス全体をコンパイル時に排除します。Ayaは`Option<T>`によるNull安全を強制し、回復可能なエラーのための`Result<T, E>`と、回復不能なパニックのためのコンパイラ追跡可能な`try fn`という、明確なデュアルモデルのエラーハンドリングシステムを提供します。

2.  **表現力豊かで親しみやすい構文 (Expressive & Familiar Syntax)**: TypeScript、Kotlin、Swiftなどの言語から来た開発者が快適に感じるクリーンな構文で、学習の障壁を下げます。可読性と定型的なコードの削減に重点を置いています。

3.  **統合されたコード品質 (Integrated Code Quality)**: 言語機能は、開発者がより良いコードを書くための指針となるべきです。Ayaは、**ディレクティブ**（`#summary`, `#panics`）を用いたファーストクラスのコンパイラ認識可能なドキュメンテーションシステムを持ち、これによりセマンティックなリントが可能になり、APIの契約が文書化され、チェックされることを保証します。

4.  **設計によるパフォーマンス (Performance by Design)**: LLVM IRにコンパイルし、Just-In-Time (JIT) コンパイラを使用することで、Ayaは高い実行パフォーマンスを実現します。これにより、高レベル言語の生産性を犠牲にすることなく、要求の厳しいアプリケーションにも対応できます。

## 機能概要

*   **型システム**: 強力で静的な名目的型付け。
*   **Null安全**: `Option<T>`型を使用し、ヌルポインタ例外を根絶。
*   **不変の徹底的な追求**: `mut`を用いて変更される可能性を明示。
*   **モダンなエラーハンドリング**: 回復可能なエラー（`Result<T, E>`と`?`演算子）と、バグに起因する回復不能なエラー（`try fn`と`panic`）の明確な区別。
*   **式ベース**: `if`や`match`のようなほとんどの制御フロー構文は、値を返す式です。
*   **オブジェクト指向**: `class`を基盤とした一貫性のあるオブジェクトモデルと、インターフェースを定義するための`protocol`。
*   **ジェネリクス**: 型パラメータ化された関数やクラスで、柔軟かつ再利用可能なコードを記述。
*   **コンパイラチェックされるドキュメント**: `#ディレクティブ`を使用してAPIの契約を記述し、コンパイラがそれを検証することで、ドキュメントが古くなるのを防ぎます。

## Hello, Aya!

Ayaのいくつかの特徴を示す短いサンプルコードです。

```zf
import { Result, Ok, Err } from "std/result";
import { Option, Some, None } from "std/option";

/// ユーザーに関する潜在的なバリデーションエラーを表します。
class UserError {
    message: String;

    /// ユーザー名を取得します。この関数は、パニックする可能性のある
    /// プログラマエラーのチェックを示すため、`try`でマークされています。
    #summary "ユーザー名を取得します。"
    #panics "内部の名前フィールドが予期せず空の場合にパニックします。これはバグを示します。"
    try fn get_name(self): String {
        if self.name.is_empty() {
            // コンストラクタが正しく使用されていれば、これは決して起こりません。
            panic("不変条件違反：Userオブジェクトの名前が空です！");
        }
        return self.name;
    }

    #summary "検証済みの名前を持つユーザーを表します。"
    #params name: "ユーザー名。空であってはなりません。"
    #returns "新しいUserまたはUserErrorを含むResult。"
    #side_effects "バリデーション中にコンソールに出力します。"
    pub fn new(name: String): Result<Self, UserError> {
        if name.is_empty() {
            return Err(UserError { message: "名前を空にすることはできません。" });
        }
        std::io::println("ユーザーは正常に検証されました！");
        Ok(Self { name })
    }
}

fn main() {
    match User::new("Alice".to_string()) {
        Ok(user) => {
            let name = try user.get_name();
            std::io::println("作成されたユーザー: " + name);
        },
        Err(error) => {
            std::io::println("ユーザーの作成に失敗しました: " + error.message);
        },
    }
}
```

## エラーハンドリング：詳細

Ayaはエラーを極めて真剣に扱い、2つの明確な経路を提供します。

### 1. 回復可能なエラー (`Result<T, E>`)

通常の状況下で失敗することが予期される操作（例：ネットワークリクエスト、ファイルI/O、入力バリデーション）については、関数は`Result<T, E>`を返すべきです。`?`演算子は、これらのエラーをクリーンに伝播させる方法を提供します。

```zf
fn process_data(): Result<Int, DataError> {
    let raw_data = fetch_data_source1()?; // Err(DataError)を伝播させる
    let processed_data = transform_data(raw_data)?;
    Ok(processed_data.value)
}
```

### 2. 回復不能なエラー (`try fn` & `panic`)

プログラムが正しい場合は**決して**起こるべきでないプログラミングエラー（例：不変条件の破損、信頼されたインデックスでの境界外アクセス）については、Ayaは`panic`を使用します。

`panic`する可能性のある関数は、**必ず**`try`キーワードでマークされなければなりません。`try`でマークされた`panic`する可能性のある関数は呼び出し元で`try`を使用して安全であることを示さなければいけません。これによって、`panic`の機会はコンパイル時にコールスタック全体を通して追跡可能になります。

```zf
// この関数はインデックスが常に有効であると想定しています。バグがこれを破る可能性があります。
try fn get_first_element(list: List<Int>): Int {
    if list.is_empty() {
        panic("空のリストの最初の要素を取得することはできません！");
    }
    return list[0];
}

// 呼び出し元も`try fn`にすることで、そのリスクを認識しなければなりません。
fn main() {
    let my_list = List::new();
    // これはパニックし、コンパイラはこの可能性を認識するよう私たちに強制しました。
    let element = try get_first_element(my_list);
}
```

## ステータス

**概念実証段階**。

## コントリビュート

あらゆる種類の貢献を歓迎します！[DEVELOPERS.md](DEVELOPERS.md)・[CONTRIBUTES.md](CONTRIBUTES.md)をご覧ください。
