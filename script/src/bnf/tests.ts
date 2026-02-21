import { it, assert, describe } from "../test";

import { parse } from "./parse";
import { analyze, Analyzer } from "./analyze";
import { generate } from "./gen";

// --- Mock Data ---
const mockTokenMap = {
    "fn": "Keyword::Fn",
    "(": "Delimiter::LeftParen",
    ")": "Delimiter::RightParen",
    "{": "Delimiter::LeftBrace",
    "}": "Delimiter::RightBrace",
    "Int": "Keyword::Int",
    "Double": "Keyword::DoubleInt",
    "let": "Keyword::Let",
    "=": "Operator::Assignment",
    ";": "Delimiter::Semicolon",
    ",": "Delimiter::Comma",
    "*": "Operator::Multiply",
    "#identifier": "Token::Identifier($Span$)",
    "#string": "Token::Literal(Literal::StringLiteral($Span$))",
};

/**
 * ユーティリティ: IRの中から特定の関数名を探す
 */
function findFunc(ir: any[], name: string) {
    return ir.find(f => f.functionName === name);
}

// ============================================================================
// 1. Parser (BNF Syntax) Tests
// ============================================================================

describe("BNF Parser - Deep Syntax", () => {
    it("should parse multiple modifiers and manual hook syntax", () => {
        const src = `
            product Pointer { "*" base: Type }
            product Id with "#identifier"
            product Str with "#string"
        `;
        const ast = parse(src);
        assert(ast.length === 3, "Should parse 3 rules");
        
        const pointer = ast[0] as any;
        assert(pointer.members[0].kind === "Terminal", "Should have terminal *");
        
        const id = ast[1] as any;
        assert(id.members[0].kind === "Terminal" && id.members[0].value === "#identifier", "Should parse manual identifier");
    });
});

// ============================================================================
// 2. Analyzer (LL(2) / Fallback / Nullable) Tests
// ============================================================================

describe("BNF Analyzer - LL(2) Disambiguation & Fallback", () => {
    it("should resolve IntType fallback (The Int vs Int Double case)", () => {
        const src = `
            branch Type { IntNormal IntDouble }
            product IntNormal { "Int" }
            product IntDouble { "Int" "Double" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const typeBranch = findFunc(ir, "Type");

        assert(typeBranch.kind === "branch", "Type should be a branch");

        assert(typeBranch.branchesJudgebleInPeek0.length === 0, "Should not be judgeable in Peek0");
        
        assert(typeBranch.branchesJudgebleInPeek1.some((b: any) => 
            b.astTypeName === "IntDouble" && b.secondTerminal === "Keyword::DoubleInt"
        ), "IntDouble should be judgeable in Peek1 via DoubleInt");

        // "Double" がなければ IntNormal にフォールバックする
        assert(typeBranch.branchesFallbackInPeek1.some((b: any) => 
            b.astTypeName === "IntNormal" && b.firstTerminal === "Keyword::Int"
        ), "IntNormal should be a fallback for Peek0(Int)");
    });

    it("should compute First(2) sets across nullable rules", () => {
        const src = `
            product Start { head: ?Opt tail: Final }
            product Opt { "let" }
            product Final { "fn" }
        `;
        // First(Start) は:
        // 1. ["let", "fn"]  (Optが存在する場合)
        // 2. ["fn"]         (Optが省略された場合)
        const ir = analyze(parse(src), mockTokenMap);
        const start = findFunc(ir, "Start");

        // Generatorがこの情報を元に、peek0が "let" なら Opt を、"fn" なら Final を選ぶコードを生成できることを期待
        assert(start.elements.length === 2, "Should have 2 elements");
    });

    it("should identify deep nullable chains", () => {
        const src = `
            product A { item: ?B }
            product B { item: ?C }
            product C { "fn" }
        `;
        const ast = parse(src);
        const analyzer = new Analyzer(ast, mockTokenMap);
        analyzer.computeNullable();
        
        assert(analyzer.nullable.has("A"), "A is nullable");
        assert(analyzer.nullable.has("B"), "B is nullable");
        assert(!analyzer.nullable.has("C"), "C is NOT nullable");
    });

    it("should handle multi-variant Peek1 conflicts by requiring backtrack", () => {
        const src = `
            branch Conflict { PathA PathB }
            product PathA { "(" "Int" ")" }
            product PathB { "(" "Int" "," }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const conflict = findFunc(ir, "Conflict");

        // k=2 では ("(", "Int") までしか見ないため、PathAとPathBは衝突する
        assert(conflict.branchesNeedBacktrack.length >= 2, "Should mark variants as needing backtrack");
        assert(conflict.branchesJudgebleInPeek1.length === 0, "Should not be able to disambiguate PathA/B in Peek1");
    });
});

// ============================================================================
// 3. Generator (Rust Code) Tests
// ============================================================================

describe("BNF Generator - Rust Code Structure", () => {
    it("should generate nested match blocks for LL(2) fallbacks", () => {
        const src = `
            branch Expr { Short Long }
            product Short { "Int" }
            product Long { "Int" "Double" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const [parserCode] = generate(ir);

        // match self.peek::<0>() { Some(Int) => { match self.peek::<1>() { ... } } } の構造を確認
        assert(parserCode.includes("match self.peek::<1>()"), "Should have nested peek1 match");
        assert(parserCode.includes("Ok(Expr::Long(self.parse_Long()?))"), "Should call parse_Long in Peek1 match");
        assert(parserCode.includes("Ok(Expr::Short(self.parse_Short()?))"), "Should call parse_Short as fallback");
    });

    it("should generate proper ArenaIter for repeated elements", () => {
        const src = `
            product Block { stmts: *Stmt }
            product Stmt { "fn" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const [_, astCode] = generate(ir);

        assert(astCode.includes("pub Stmt: ArenaIter<Stmt>"), "Should use ArenaIter for list members");
    });

    it("should mark manual hooks with skip-logic in parser", () => {
        const src = `product Identifier with "#identifier"`;
        const ir = analyze(parse(src), mockTokenMap);
        const [parserCode] = generate(ir);

        // manual hook は trait 内でメソッド定義のみ（実装は無し）になる
        assert(parserCode.trim().includes("fn parse_Identifier(&mut self) -> Result<Identifier, Self::Error>;"), 
            "Manual product should be generated as abstract method");
    });

    it("should produce exhaustive match arms for branch errors", () => {
        const src = `
            branch Top { A B }
            product A { "let" }
            product B { "fn" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const [parserCode] = generate(ir);

        // _ => Err(...) が生成されているか
        assert(parserCode.includes("_ => Err(Self::Error::build"), "Branch match should be exhaustive with error arm");
    });
});

console.log("\n✅ All parser generater tests passed");
