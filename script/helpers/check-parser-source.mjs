import { readFileSync } from "fs";

const PARSER_SOURCECODE_PATH = process.cwd() + "/src/parser/parser.rs";


/**
 * check if all parse functions are implemented in "src/parser/parser.rs"
 * @param {string[]} parseFunctions 
 */
export function checkParseFunctionsIsImplemented(parseFunctions) {
    const parserSource = readFileSync(PARSER_SOURCECODE_PATH, "utf-8");
    const isOk = parseFunctions.every(func => {
        if (parserSource.includes(func.substring(0, func.indexOf("unimplemented!()")))) {
            return true;
        } else {
            console.error(`not found: \n${func.substring(func.indexOf("fn parse_"), func.indexOf("("))}`);
            return false;
        }
    });
    if (!isOk) { throw new Error("not all functions are implemented.") }
}
