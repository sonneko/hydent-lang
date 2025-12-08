import { readFile } from "./sys.js";

const PARSER_SOURCECODE_PATH = "/src/parser/parser.rs";


/**
 * check if all parse functions are implemented in "src/parser/parser.rs"
 * @param {string[]} parseFunctions 
 */
export async function checkParseFunctionsIsImplemented(parseFunctions) {
    let isImplementd = true;
    const parserSource = await readFile(PARSER_SOURCECODE_PATH, "utf-8");
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
