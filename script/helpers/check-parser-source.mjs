import { readFileSync } from "fs";

const PARSER_SOURCECODE_PATH = process.cwd() + "/src/parser/parser.rs";


/**
 * check if all parse functions are implemented in "src/parser/parser.rs"
 * @param {string[]} parseFunctions 
 */
export function checkParseFunctionsIsImplemented(parseFunctions) {
    let isImplementd = true;
    const parserSource = readFileSync(PARSER_SOURCECODE_PATH, "utf-8");
    parseFunctions.forEach(func => {
        if (!parserSource.includes(func.substring(0, func.indexOf(" unimplemented!()")))) {
            console.log(`not found function ${func}`);
            isImplementd = false;
        }
    });
    if (!isImplementd) {
        process.exit(1);
    }
}
