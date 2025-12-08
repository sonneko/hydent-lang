import { exec, writeFile } from "./helpers/sys.js";
import { parseGrammerBNF } from "./helpers/bnf-analysis.js";
import { generateParserFunctions } from "./helpers/parser-fn-generete.js";
import { checkParseFunctionsIsImplemented } from "./helpers/check-parser-source.js";
import { checkAllCharsDefined } from "./helpers/bnf-check.js";

const GENERATED_NON_TERMINAL_CHARACTERS = `/spec/system/non-terminal-characters.txt`;
const GENERATED_TERMINAL_CHARACTERS = `/spec/system/terminal-characters.txt`;
const GENERATED_PARSER_FUNCTIONS = `/spec/system/parser-functions.rs.txt`;


export async function check() {

    console.log("🟦 Started check on frontend logic.")

    try {
        const grammer = await parseGrammerBNF();
        console.log("\t✅ Parsed EBNF grammer file.")

        const parseFunctions = generateParserFunctions(grammer.rules);
        checkAllCharsDefined(grammer.rules);
        console.log("\t✅ All chars are defined in EBNF grammer file.");

        const functionsStr = parseFunctions.join("\n\n");
        await writeFile(GENERATED_PARSER_FUNCTIONS, functionsStr);
        console.log("\t✅ Generated parser functions in 'spec/parser-functions.rs.txt'")

        await checkParseFunctionsIsImplemented(parseFunctions);
        console.log("\t✅ all functions are implemented in 'src/parser/parser.rs'");

        const nonTerminalChars = grammer.nonTerminalChars.join("\n");
        await writeFile(GENERATED_NON_TERMINAL_CHARACTERS, nonTerminalChars);
        console.log("\t✅ Generated non-terminal characters in 'spec/non-terminal-characters.txt'");

        // TODO: add checking if all non-terminal characters are defined in AST type definition in compiler source.

        const terminalChars = [...new Set(grammer.terminalChars)].join("\n");
        await writeFile(GENERATED_TERMINAL_CHARACTERS, terminalChars);
        console.log("\t✅ Generated terminal characters in 'spec/terminal-characters.txt'");

        // TODO: add checking if all terminal characters are defined in Token definition in compiler source.

        const result = exec("cargo check");
        if (result.stderr) {
            throw new Error("\t❌ Error occured during check on Rust code.");
        }
        console.log("✅ Checked on Rust code.");

        // TODO: add rust test for tokenizer, parser and so on.

        // TODO: add checking code for compiler frontend.

    } catch (err) {
        console.log("\t❌ Error occured during check on EBNF and parser.")
        console.log("\tError:\t" + err.message.replace("\n", "\n\t\t"));
        process.exit(1);
    }




}