import fs from "fs";
import { parse } from "./bnf/parse";
import { analyze } from "./bnf/analyzer";
import { generateParser } from "./bnf/gen";

const BNF_FILE_PATH = "../assets/grammer.txt";
const AST_FILE_PATH = "../spec/frontend/ast.json";
const IR_FILE_PATH = "../spec/frontend/ir.json"

function main() {
    console.log("🤖 Generating AST...")
    const source = fs.readFileSync(BNF_FILE_PATH, "utf8");
    const ast = parse(source);
    fs.writeFileSync(AST_FILE_PATH, JSON.stringify(ast, null, 2), "utf8");
    console.log("✅ AST written to spec/frontend/ast.json");

    console.log("🤖 Analyzing AST...");
    const analysis = analyze(ast);
    fs.writeFileSync(IR_FILE_PATH, JSON.stringify(analysis, (_, value) => {
        if (value instanceof Map) {
            return Object.fromEntries(value); // または Array.from(value.entries())
        }
        if (value instanceof Set) {
            return Array.from(value); // または [...value]
        }
        return value;
    }, 2), "utf8");
    console.log("✅ IR written to spec/frontend/ir.json");

    console.log("🤖 Generating Parser...");
    const parser = generateParser(analysis);
    fs.writeFileSync("../src/parser/generated_parser.rs", parser, "utf8");
    console.log("✅ Parser written to src/parser/generated_parser.rs");

}

main();
