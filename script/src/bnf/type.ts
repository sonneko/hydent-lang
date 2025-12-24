// AST type definition of Bnf

// <bnf> ::= { <character> "::=" <alternative> }
// <alternative> ::= <term> { "|" <term> }
// <term> ::= <factor> { <factor> }
// <factor> ::= <character>
//            | <terminal_string>
//            | "[" <alternative> "]"
//            | "{" <alternative> "}"
//            | "(" <alternative> ")"
// <character> ::= "<" <STRING> ">"
// <terminal_string> ::= "\"" <STRING> "\""

export type BNF_AST = {
    rules: BNF_Rule[];
}

export type BNF_Rule = {
    name: string;
    alternative: BNF_Alternative;
}

export type BNF_Alternative = {
    terms: BNF_Term[];
}

export type BNF_Term = {
    factors: BNF_Factor[];
}

export type BNF_Factor = {
    node_type: "character";
    character: string;
} | {
    node_type: "terminal_string";
    terminal_string: string;
} | {
    node_type: "option";
    alternative: BNF_Alternative;
} | {
    node_type: "repeat";
    alternative: BNF_Alternative;
} | {
    node_type: "group";
    alternative: BNF_Alternative;
}


export type TokenType =
    | "ASSIGN"          // ::=
    | "PIPE"            // |
    | "LBRACKET"        // [
    | "RBRACKET"        // ]
    | "LBRACE"          // {
    | "RBRACE"          // }
    | "LPAREN"          // (
    | "RPAREN"          // )
    | "SEMICOLON"       // ;
    | "TERMINAL"        // "string"
    | "NON_TERMINAL"    // <string>
    | "EOF";

export type Token = {
    type: TokenType;
    value: string;
    line: number;
    col: number;
};
