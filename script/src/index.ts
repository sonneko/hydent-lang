import fs from "fs";
import { execSync } from 'child_process';

import { parse } from "./parser_gen/parse";
import { analyze } from "./parser_gen/analyze";
import { generate } from "./parser_gen/gen";
import { generateASTSizeCheckerRustTest, parseASTSizeCheckerResult } from "./parser_gen/check_ast_size";
import { astName } from "./parser_gen/ir";
import { generateTokenTypeMap } from "./parser_gen/tokenmap_gen";

const BNF_FILE_PATH = "../assets/grammer.txt";
const AST_FILE_PATH = "../spec/frontend/ast.json";
const IR_FILE_PATH = "../spec/frontend/ir.json";
const PARSER_FILE_PATH = "../src/parser/generated_parser.rs";
const AST_TYPE_FILE_PATH = "../src/parser/generated_ast.rs";
const AST_SIZE_CHECKER_FILE_PATH = "../src/parser/tests/ast_size_checker.rs";
const AST_SIZE_REPORT_FILE_PATH = "../spec/frontend/ast_size.json";
const TOKEN_MAP_FILE_PATH = "../src/tokenizer/generated_tokenmap.rs"

function main() {
    fs.mkdirSync("../spec/frontend", { recursive: true });

    const mode = process.argv[2] === "ci" ? "ci" : "dev";

    console.log("🤖 Generating TokenMap...");
    const tokenMapCode = generateTokenTypeMap();
    fs.writeFileSync(TOKEN_MAP_FILE_PATH, tokenMapCode, "utf8");
    console.log("✅ TokenMap written to src/tokenizer/generated_tokenmap.rs");

    console.log("🤖 Generating AST...");
    const source = fs.readFileSync(BNF_FILE_PATH, "utf8");
    const ast = parse(source);
    fs.writeFileSync(AST_FILE_PATH, JSON.stringify(ast, null, 2), "utf8");
    console.log("✅ AST written to spec/frontend/ast.json");

    console.log("🤖 Analyzing AST...");
    const analysis = analyze(ast);
    fs.writeFileSync(IR_FILE_PATH, JSON.stringify(analysis, (_, value) => {
        if (value instanceof Map) {
            return Object.fromEntries(value);
        }
        if (value instanceof Set) {
            return [...value];
        }
        return value;
    }, 2), "utf8");
    console.log("✅ IR written to spec/frontend/ir.json");

    console.log("🤖 Generating Parser...");
    const parser = generate(analysis);
    fs.writeFileSync(PARSER_FILE_PATH, parser[0], "utf8");
    fs.writeFileSync(AST_TYPE_FILE_PATH, parser[1], "utf8");
    console.log("✅ Parser and AST type definition written to src/parser/generated_parser.rs and generated_ast.rs");

    if (mode === "ci") {
        console.log("🤖 Checking AST type size...");
        fs.writeFileSync(AST_SIZE_CHECKER_FILE_PATH, generateASTSizeCheckerRustTest(ast.map(node => astName(node.name))));
        console.log("* Checker rust test written to src/parser/tests/ast_size_checker.rs");
        let sizeChekerResult: string;
        try {
            sizeChekerResult = execSync("cargo test report_ast_size --verbose -- --ignored --nocapture", {
                encoding: "utf-8",
                stdio: ['pipe', 'pipe', 'ignore']
            }).toString();
        } catch (e) {
            throw new Error("⚠️ AST type size checker rust tests failed");
        }
        console.log("* AST type size checker rust tests runned");
        const astSizes = parseASTSizeCheckerResult(sizeChekerResult);
        fs.writeFileSync(AST_SIZE_REPORT_FILE_PATH, JSON.stringify(astSizes, null, 2), "utf8")
        console.log("* AST type size report written to spec/frontend/ast_size.json");
        const tooBigSizeAsts = astSizes.filter(({ size }) => size > 1024 /* 128 Byte */);
        if (tooBigSizeAsts.length === 0) {
            console.log("✅ Checked if no AST types have too big sizes");
        } else {
            console.log("⚠️ " + tooBigSizeAsts.map(({ ast }) => ast).join(", ") + " has too big size.")
            console.warn("⚠️ AST types have too big sizes. see spec/frontend/ast_size.json and add `with \"box\"` to grammar.txt");
        }
    }

    if (mode === "ci") {
        console.log("🤖 formatting generated rust files ")
        try {
            execSync(`rustfmt ${PARSER_FILE_PATH} ${AST_TYPE_FILE_PATH} ${AST_SIZE_CHECKER_FILE_PATH} ${TOKEN_MAP_FILE_PATH}`);
        } catch (e) {
            console.warn(`⚠️ rustfmt not found or failed because of syntax error`)
        }
        console.log("✅ formatted generated rust files");
    }
}

main();
