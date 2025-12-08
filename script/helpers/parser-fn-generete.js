import { toUpperCamelCase } from "./lib.js";

/**
 * generate parser functions from grammer rules.
 * @param {{ left: string, right: string[], raw: string }[]} rules 
 * @returns { string[] }
 */
export function generateParserFunctions(rules) {
    const parseFunctions = rules.map(item => (
        `    /// \`\`\`ebnf\n/// ${item.raw.split("\n").filter(item => item.trim().length !== 0).join("\n///")}\n/// \`\`\`\nfn parse_${item.left}(&mut self) -> Return<AST::${toUpperCamelCase(item.left)}> { unimplemented!(); }`
            .split("\n").join("\n    ")
    ));
    return parseFunctions;
}
