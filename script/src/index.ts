import fs from "fs";
import { execSync } from 'child_process';

import { parse } from "./bnf/parse";
import { analyze } from "./bnf/analyze";
import { generateParser } from "./bnf/gen";

const BNF_FILE_PATH = "../assets/grammer.txt";
const AST_FILE_PATH = "../spec/frontend/ast.json";
const IR_FILE_PATH = "../spec/frontend/ir.json";
const PARSER_FILE_PATH = "../src/parser/generated_parser.rs";
const AST_TYPE_FILE_PATH = "../src/parser/generated_ast.rs";

function main() {
    fs.mkdirSync("../spec/frontend", { recursive: true });

    console.log("🤖 Generating AST...")
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
            return Array.from(value);
        }
        return value;
    }, 2), "utf8");
    console.log("✅ IR written to spec/frontend/ir.json");

    console.log("🤖 Generating Parser...");
    const parser = generateParser(analysis);
    fs.writeFileSync(AST_TYPE_FILE_PATH, parser[0], "utf8");
    fs.writeFileSync(PARSER_FILE_PATH, parser[1], "utf8");
    execSync(`rustfmt ${PARSER_FILE_PATH} ${AST_TYPE_FILEPATH}`);
    console.log("✅ Parser and AST type definition written to src/parser/generated_parser.rs and generated_ast.rs");

}

main();
