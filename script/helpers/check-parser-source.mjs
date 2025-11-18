import { readFileSync } from "fs";

const PARSER_SOURCECODE_PATH = process.cwd() + "/src/parser/parser.rs";


/**
 * check if all parse functions are implemented in "src/parser/parser.rs"
 * @param {string[]} parseFunctions 
 */
export function checkParseFunctionsIsImplemented(parseFunctions) {
    const parserSource = readFileSync(PARSER_SOURCECODE_PATH, "utf-8");
    parseFunctions.forEach(func => {
        if (!parserSource.includes(func.substring(0, func.indexOf("unimplemented!()")))) {
            throw new Error(`not found function ${func.substring(func.indexOf("fn parse_"), func.indexOf("("))}` + "\nnot all functions are implemented.");
        }
    });
}
