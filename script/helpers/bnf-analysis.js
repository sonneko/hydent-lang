import { readFile } from "./sys.js";
import { assert } from "./sys.js";

const GRAMMER_FILE_PATH = "/assets/grammer.bnf";


/**
 * parse grammer from "spec/grammer.bnf" and return results.
 * @returns { { nonTerminalChars: string[], terminalChars: string[] rules: { left: string, right: string[], raw: string }[] } }
 */
export async function parseGrammerBNF() {
    const rules = split2Rules(skipSharpComment(await getGrammerFile())).map(item => ({
        left: split2Character(item.left).map((item, index) => {
            assert(index == 0);
            return item;
        })[0],
        right: (() => {
            const stringLiterals = item.right.split("\"").filter((_, i) => i % 2 == 1);
            const notStringLiterals = item.right.split("\"").filter((_, i) => i % 2 == 0).join("");
            return [
                ...split2Character(notStringLiterals.trim()),
                ...stringLiterals.map(item => `"${item.trim()}"`)
            ];
        })(),
        raw: item.left + "::=" + item.right,
    }));
    const nonTerminalChars = rules.map(item => item.left);
    const terminalChars = [
        ...new Set(
            rules.map(item => item.right)
                .reduce((pre, curr) => [...pre, ...curr], [])
        )
    ].filter(item => !nonTerminalChars.includes(item))
        .sort((a, b) => b.length - a.length);
    return {
        nonTerminalChars,
        terminalChars,
        rules,
    };
}

/**
 * Return grammer file content as string.
 * @returns {string}
 */
async function getGrammerFile() {
    return await readFile(GRAMMER_FILE_PATH);
}

/**
 * Skip sharp comments in grammer file.
 * @param {string} fileContent 
 * @returns {string}
 */
function skipSharpComment(fileContent) {
    const lines = fileContent.split("\n");
    return lines.filter(line => !line.startsWith("#")).join("\n");
}

/**
 * Split grammer file to rules.
 * @param {string} fileContent 
 * @returns { { left: string, right: string }[] }
 */
function split2Rules(fileContent) {
    return fileContent.split("$").filter((_, index) => index != 0).map(item => ({
        left: item.split("::=")[0],
        right: item.split("::=")[1],
    }));
}

/**
 * Split string to BNF characters.
 * @param {string} str 
 * @returns {string[]}
 */
function split2Character(str) {
    let chars = [];
    let i = 0;
    while (i < str.length) {
        if (str[i] === "<") {
            const startOfContent = i + 1;
            let endOfContent = str.indexOf(">", startOfContent);
            if (endOfContent !== -1) {
                chars.push(str.substring(startOfContent, endOfContent));
                i = endOfContent + 1;
            } else {
                break;
            }
        } else {
            i++;
        }
    }
    return chars;
}
