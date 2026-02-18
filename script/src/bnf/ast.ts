// ```
// <bnf> ::= { <rule> }
// 
// <rule> ::= <branch_rule> | <product_rule>
// 
// <branch_rule> ::= "branch" <identifier> "{" <branch_rule_inner> "}"
// <branch_rule_inner> ::= { <identifier> [ "with" <string_literal> ] }
// 
// <product_rule> ::= "product" <identifier> ( "{" <product_inner> "}" )?
// <product_inner> ::= { <product_item> [ "with" <string_literal> ] }
// <product_item> ::= ( <identifier> ":" <nonterminal> ) | <terminal>
// <nonterminal> ::= <repete_item> | <option_item> | <item>
// <repete_item> ::= "*" <item>
// <option_item> ::= "?" <item>
// <item> ::= <identifier>
// <terminal> ::= <string_literal>
// ```

export type TypeModifier = 'None' | 'List' | 'Option';

export interface TypeReference {
    name: string;
    modifier: TypeModifier;
}

export interface BranchVariant {
    name: string;
    note: string | null;
}

export type ProductMember =
    | { kind: 'Field'; name: string; type: TypeReference; note: string | null }
    | { kind: 'Terminal'; value: string; note: string | null };

export interface BranchRule {
    kind: 'Branch';
    name: string;
    variants: BranchVariant[];
}

export interface ProductRule {
    kind: 'Product';
    name: string;
    members: ProductMember[];
}

export type Rule = BranchRule | ProductRule;

export type Grammar = Rule[];

// --- Lexer Types & Implementation ---

export type TokenKind =
    | 'Branch' | 'Product' | 'With'
    | 'LBrace' | 'RBrace' | 'Colon' | 'Star' | 'Question'
    | 'Identifier' | 'StringLiteral' | 'EOF';

export type Token =
    | { kind: 'Branch' }
    | { kind: 'Product' }
    | { kind: 'With' }
    | { kind: 'LBrace' }
    | { kind: 'RBrace' }
    | { kind: 'Colon' }
    | { kind: 'Star' }
    | { kind: 'Question' }
    | { kind: 'Identifier'; value: string }
    | { kind: 'StringLiteral'; value: string }
    | { kind: 'EOF' };

export const TOKEN_PATTERNS: Array<{ kind: TokenKind; regex: RegExp }> = [
    { kind: 'Branch', regex: /^branch\b/ },
    { kind: 'Product', regex: /^product\b/ },
    { kind: 'With', regex: /^with\b/ },
    { kind: 'LBrace', regex: /^\{/ },
    { kind: 'RBrace', regex: /^\}/ },
    { kind: 'Colon', regex: /^:/ },
    { kind: 'Star', regex: /^\*/ },
    { kind: 'Question', regex: /^\?/ },
    { kind: 'StringLiteral', regex: /^"([^"]*)"/ }, // グループキャプチャあり
    { kind: 'Identifier', regex: /^[a-zA-Z_][a-zA-Z0-9_]*/ },
];
