import { parse } from "./parse";
import { analyze } from "./analyze";
import { generate } from "./gen";

/**
 * 簡易テストランナー
 */
function describe(name: string, fn: () => void) {
    console.log(`\n=== Testing: ${name} ===`);
    fn();
}

function it(name: string, fn: () => void) {
    try {
        fn();
        console.log(`✅ ${name}`);
    } catch (e) {
        console.error(`❌ ${name}`);
        console.error(e);
        process.exit(1);
    }
}

function assert(condition: boolean, message: string) {
    if (!condition) throw new Error(message);
}

// ----------------------------------------------------------------------------
// Mock Data
// ----------------------------------------------------------------------------
const mockTokenMap = {
    "(": "Delimiter::ParenthesisOpen",
    ")": "Delimiter::ParenthesisClose",
    "{": "Delimiter::BraceOpen",
    "}": "Delimiter::BraceClose",
    ";": "Delimiter::Semicolon",
    "let": "Keyword::Let",
    "fn": "Keyword::Fn",
    "=": "Operator::Assign",
    "+": "Operator::Plus",
};

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

describe("BNF Parser (Lexer & Parser)", () => {
    it("should parse a simple product rule", () => {
        const src = `product VariableDecl { "let" name: Identifier "=" value: Expression ";" }`;
        const ast = parse(src);
        
        assert(ast.length === 1, "Should have 1 rule");
        const rule = ast[0] as any;
        assert(rule.kind === "Product", "Rule should be Product");
        assert(rule.name === "VariableDecl", "Name mismatch");
        assert(rule.members.length === 5, "Should have 5 members (3 terminals, 2 fields)");
    });

    it("should parse a branch rule with notes", () => {
        const src = `
            branch Statement {
                VariableDecl with "let variable"
                ReturnStmt
            }
        `;
        const ast = parse(src);
        assert(ast[0].kind === "Branch", "Rule should be Branch");
        const branch = ast[0] as any;
        assert(branch.variants.length === 2, "Should have 2 variants");
        assert(branch.variants[0].note === "let variable", "Note mismatch");
    });

    it("should parse modifiers (*, ?)", () => {
        const src = `product Params { head: ?Item tail: *Item }`;
        const ast = parse(src);
        const members = (ast[0] as any).members;
        assert(members[0].type.modifier === "Option", "? should be Option");
        assert(members[1].type.modifier === "List", "* should be List");
    });
});

describe("BNF Analyzer (Nullable & First/Follow)", () => {
    it("should correctly identify nullable rules", () => {
        const src = `
            product OptionalPart { innter: ?Inner }
            product Inner { "fn" }
            product ListPart { innter: *Inner }
            branch Mixed { OptionalPart Inner }
        `;
        const ast = parse(src);
        const ir = analyze(ast, mockTokenMap);

        // analyze.ts の内部状態を直接見ることはできないが、
        // 生成されたIRの結果から間接的に nullable の影響を推論できる。
        // ここではクラッシュせずに解析が完了することを最低限保証する。
        assert(ir.length === 4, "IR should be generated for all rules");
    });

    it("should distinguish branches by Peek0 (LL(1))", () => {
        const src = `
            branch Top { A B }
            product A { "let" }
            product B { "fn" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const top = ir.find(f => f.astTypeName === "Top") as any;

        assert(top.kind === "branch", "Top should be a branch");
        assert(top.branchesJudgebleInPeek0.length === 2, "Both variants should be decidable by Peek0");
        assert(top.branchesJudgebleInPeek1.length === 0, "No Peek1 needed");
    });

    it("should distinguish branches by Peek1 (LL(2))", () => {
        const src = `
            branch Top { CommonA CommonB }
            product CommonA { "(" "let" }
            product CommonB { "(" "fn" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const top = ir.find(f => f.astTypeName === "Top") as any;

        // "(" が共通なので Peek0 では判定できない
        assert(top.branchesJudgebleInPeek0.length === 0, "Should not be decidable by Peek0");
        // Peek1 で "let" と "fn" を見て判定する
        assert(top.branchesJudgebleInPeek1.length === 2, "Should be decidable by Peek1");
    });

    it("should fallback to backtracking when LL(2) is insufficient", () => {
        const src = `
            branch Ambiguous { Way1 Way2 }
            product Way1 { "(" "let" "=" }
            product Way2 { "(" "let" "+" }
        `;
        // k=2 (analyze.tsの実装) では "(" "let" までしか見ないので衝突する
        const ir = analyze(parse(src), mockTokenMap);
        const top = ir.find(f => f.astTypeName === "Ambiguous") as any;

        assert(top.branchesNeedBacktrack.length > 0, "Should require backtracking");
    });
});

describe("BNF Generator", () => {
    it("should generate valid-looking Rust code structure", () => {
        const src = `product Simple { "let" name: Id }`;
        const ir = analyze(parse(src), mockTokenMap);
        const [parserCode, astCode] = generate(ir);

        // Parser code check
        assert(parserCode.includes("pub trait GeneratedParser"), "Missing trait definition");
        assert(parserCode.includes("fn parse_Simple"), "Missing function definition");
        assert(parserCode.includes("self.expect(Keyword::Let)?"), "Missing terminal expectation");

        // AST code check
        assert(astCode.includes("pub struct Simple"), "Missing struct definition");
        assert(astCode.includes("impl ASTNode for Simple"), "Missing trait impl");
    });

    it("should generate enums for branch rules", () => {
        const src = `
            branch Expr { Add Sub }
            product Add { "add" }
            product Sub { "sub" }
        `;
        const ir = analyze(parse(src), mockTokenMap);
        const [_, astCode] = generate(ir);

        assert(astCode.includes("pub enum Expr {"), "Enum Expr should be defined");
        assert(astCode.includes("Add(Add),"), "Enum variant Add should exist");
        assert(astCode.includes("Sub(Sub),"), "Enum variant Sub should exist");
        assert(astCode.includes("Invalid,"), "Invalid variant should exist for error recovery");
    });
});

console.log("\nAll BNF tests passed successfully!");
