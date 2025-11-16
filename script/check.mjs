import { writeFileSync } from "fs";
import { parseGrammerBNF } from "./helpers/bnf-analysis.mjs";
import { generateParserFunctions } from "./helpers/parser-fn-generete.mjs";
import { checkParseFunctionsIsImplemented } from "./helpers/check-parser-source.mjs";
import { checkAllCharsDefined } from "./helpers/bnf-check.mjs";

const GENERATED_NON_TERMINAL_CHARACTERS = `${process.cwd()}/spec/non-terminal-characters.txt`;
const GENERATED_TERMINAL_CHARACTERS = `${process.cwd()}/spec/terminal-characters.txt`;
const GENERATED_PARSER_FUNCTIONS = `${process.cwd()}/spec/parser-functions.rs.txt`;


console.log("🟦 Started check on EBNF and parser.")

const grammer = parseGrammerBNF();
console.log("\t✅ Parsed EBNF grammer file.")

const parseFunctions = generateParserFunctions(grammer.rules);
checkAllCharsDefined(grammer.rules);
console.log("\t✅ All chars are defined in EBNF grammer file.");

const functionsStr = parseFunctions.join("\n\n");
writeFileSync(GENERATED_PARSER_FUNCTIONS, functionsStr);
console.log("\t✅ Generated parser functions in 'spec/parser-functions.rs.txt'")

checkParseFunctionsIsImplemented(parseFunctions);
console.log("\t✅ all functions are implemented in 'src/parser/parser.rs'");

const nonTerminalChars = grammer.nonTerminalChars.join("\n");
writeFileSync(GENERATED_NON_TERMINAL_CHARACTERS, nonTerminalChars);
console.log("\t✅ Generated non-terminal characters in 'spec/non-terminal-characters.txt'");

const terminalChars = [...new Set(grammer.terminalChars)].join("\n");
writeFileSync(GENERATED_TERMINAL_CHARACTERS, terminalChars);
console.log("\t✅ Generated terminal characters in 'spec/terminal-characters.txt'");

// TODO: add checking code for compiler.
