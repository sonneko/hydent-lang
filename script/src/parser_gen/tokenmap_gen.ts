import { TokenMap } from "./analyze";

export function generateTokenTypeMap() {
    let ret = "";
    const allTokens = Object.entries(new TokenMap().getAll());
    ret += `use phf::phf_map;\n`;
    ret += `use crate::tokenizer::errors::TokenizeErr;\n`;
    ret += `use crate::tokenizer::tokenize::Tokenizer;\n`;
    ret += `use crate::tokenizer::tokens::{Token, Keyword, Operator, Delimiter};\n\n`;
    ret += `pub static LONG_KEYWORDS_MAP: phf::Map<&'static [u8], Token> = phf_map!{\n`;
    const tokensPhf = allTokens
        .filter(([literal, _]) => !literal.includes("#") && literal.length >= 5)
        .sort((pre, curr) => curr[0].length - pre[0].length);
    for (const [literal, token] of tokensPhf) {
        ret += `    b"${literal}" => ${token.replace("$", "").replace("$", "")},\n`;
    }
    ret += `};\n\n`;

    ret += `pub fn scan_short_keywords(literal: &[u8]) -> Token {\n`;
    ret += `    match literal {\n`;
    const tokensMatchKeywords = allTokens
        .filter(([literal, token]) => !literal.includes("#") && literal.length < 5 && token.includes("Keyword"))
        .sort((pre, curr) => curr[0].length - pre[0].length);
    for (const [literal, token] of tokensMatchKeywords) {
        ret += `        b"${literal}" => ${token.replace("$", "").replace("$", "")},\n`;
    }
    ret += `        _ => Token::Invalid,\n`;
    ret += `    }\n`;
    ret += `}\n\n`;

    const operators = allTokens.filter(([literal, token]) => 
        !literal.includes("#") && 
        (token.includes("Operator") || token.includes("Delimiter"))
    );
    ret += `pub fn scan_operator_or_delimiter(tokenizer: &mut Tokenizer<'_, '_>) -> Result<Token, TokenizeErr> {\n`;
    ret += `    let start_pos = tokenizer.current_pos;\n`;
    ret += `    let b = tokenizer.peek();\n`;
    ret += `    match b {\n`;
    const tree = buildTree(operators, 0);
    ret += generateMatchArms(tree, 1);
    ret += `        _ => Err(TokenizeErr::UnknownToken(start_pos)),\n`;
    ret += `    }\n`;
    ret += `}\n`;
    
    return ret;
}


type TokenData = [string, string]; // [literal, token_variant]

interface TreeNode {
    token?: string;
    children: Map<number, TreeNode>;
}

function buildTree(tokens: TokenData[], index: number): TreeNode {
    const node: TreeNode = { children: new Map() };
    for (const [literal, token] of tokens) {
        if (index === literal.length) {
            node.token = token.replace(/\$/g, "");
            continue;
        }
        const charCode = literal.charCodeAt(index);
        if (!node.children.has(charCode)) {
            node.children.set(charCode, buildTree([], index + 1));
        }
        const child = node.children.get(charCode)!;
        addToTree(child, literal, token, index + 1);
    }
    return node;
}

function addToTree(node: TreeNode, literal: string, token: string, index: number) {
    if (index === literal.length) {
        node.token = token.replace(/\$/g, "");
        return;
    }
    const charCode = literal.charCodeAt(index);
    if (!node.children.has(charCode)) {
        node.children.set(charCode, { children: new Map() });
    }
    addToTree(node.children.get(charCode)!, literal, token, index + 1);
}

function generateMatchArms(node: TreeNode, indent: number): string {
    let res = "";
    const space = "    ".repeat(indent);

    const sortedKeys = Array.from(node.children.keys()).sort((a, b) => a - b);

    for (const charCode of sortedKeys) {
        const char = String.fromCharCode(charCode);
        const child = node.children.get(charCode)!;
        
        res += `${space}Some(b'${char}') => {\n`;
        res += `${space}    tokenizer.advance();\n`;
        
        if (child.children.size > 0) {
            res += `${space}    match tokenizer.peek() {\n`;
            res += generateMatchArms(child, indent + 2);
            res += `${space}        _ => {\n`;
            if (child.token) {
                res += `${space}            Ok(${child.token})\n`;
            } else {
                res += `${space}            Err(TokenizeErr::UnknownToken(tokenizer.current_pos))\n`;
            }
            res += `${space}        }\n`;
            res += `${space}    }\n`;
        } else {
            res += `${space}    Ok(${child.token})\n`;
        }
        res += `${space}}\n`;
    }
    return res;
}
