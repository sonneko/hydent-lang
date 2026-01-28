export class Tokenizer {
    private static readonly RULES: [TokenType, RegExp][] = [
        ['KEYWORD_BRANCH', /^branch\b/],
        ['KEYWORD_PRODUCT', /^product\b/],
        ['KEYWORD_WITH', /^with\b/],
        ['LBRACE', /^{/],
        ['RBRACE', /^}/],
        ['COLON', /^:/],
        ['STAR', /^\*/],
        ['QUESTION', /^\?/],
        ['IDENTIFIER', /^[a-zA-Z_]\w*/],
        ['STRING_LITERAL', /^"([^"\\]|\\.)*"/], // シンプルなダブルクォート文字列
    ];

    tokenize(input: string): Token[] {
        let str = input.trim();
        const tokens: Token[] = [];

        while (str.length > 0) {
            let matched = false;
            for (const [type, regex] of Tokenizer.RULES) {
                const match = str.match(regex);
                if (match) {
                    tokens.push({ type, value: match[0] });
                    str = str.slice(match[0].length).trim();
                    matched = true;
                    break;
                }
            }
            if (!matched) throw new Error(`Unexpected character at: ${str.slice(0, 10)}...`);
        }
        tokens.push({ type: 'EOF', value: '' });
        return tokens;
    }
}


// --- Parser ---
export class Parser {
    private tokens: Token[];
    private pos = 0;

    constructor(tokens: Token[]) {
        this.tokens = tokens;
    }

    private peek() { return this.tokens[this.pos]; }

    private consume(expected?: TokenType): Token {
        const token = this.peek();
        if (expected && token.type !== expected) {
            throw new Error(`Expected ${expected} but found ${token.type} at position ${this.pos}`);
        }
        this.pos++;
        return token;
    }

    // <bnf> ::= { <rule> }
    parseBNF() {
        const rules = [];
        while (this.peek().type !== 'EOF') {
            rules.push(this.parseRule());
        }
        return { type: 'Program', rules };
    }

    // <rule> ::= <branch_rule> | <product_rule>
    private parseRule() {
        const token = this.peek();
        if (token.type === 'KEYWORD_BRANCH') return this.parseBranchRule();
        if (token.type === 'KEYWORD_PRODUCT') return this.parseProductRule();
        throw new Error(`Invalid rule start: ${token.value}`);
    }

    // <branch_rule> ::= "branch" <identifier> "{" <branch_rule_inner> "}"
    private parseBranchRule() {
        this.consume('KEYWORD_BRANCH');
        const id = this.consume('IDENTIFIER').value;
        this.consume('LBRACE');
        const items = [];
        while (this.peek().type !== 'RBRACE') {
            items.push(this.parseBranchRuleInner());
        }
        this.consume('RBRACE');
        return { type: 'BranchRule', id, items };
    }

    private parseBranchRuleInner() {
        const id = this.consume('IDENTIFIER').value;
        let metadata = null;
        if (this.peek().type === 'KEYWORD_WITH') {
            this.consume('KEYWORD_WITH');
            metadata = this.consume('STRING_LITERAL').value;
        }
        return { id, metadata };
    }

    // <product_rule> ::= "product" <identifier> "{" <product_inner> "}"
    private parseProductRule() {
        this.consume('KEYWORD_PRODUCT');
        const id = this.consume('IDENTIFIER').value;
        this.consume('LBRACE');
        const items = [];
        while (this.peek().type !== 'RBRACE') {
            items.push(this.parseProductInner());
        }
        this.consume('RBRACE');
        return { type: 'ProductRule', id, items };
    }

    private parseProductInner() {
        const item = this.parseProductItem();
        let metadata = null;
        if (this.peek().type === 'KEYWORD_WITH') {
            this.consume('KEYWORD_WITH');
            metadata = this.consume('STRING_LITERAL').value;
        }
        return { ...item, metadata };
    }

    private parseProductItem() {
        const token = this.peek();
        if (token.type === 'STRING_LITERAL') {
            return { kind: 'terminal', value: this.consume().value };
        } else {
            const id = this.consume('IDENTIFIER').value;
            this.consume('COLON');
            const nonTerminal = this.parseNonTerminal();
            return { kind: 'nonterminal_mapping', id, nonTerminal };
        }
    }

    private parseNonTerminal() {
        const token = this.peek();
        if (token.type === 'STAR') {
            this.consume();
            return { modifier: 'repeat', item: this.consume('IDENTIFIER').value };
        }
        if (token.type === 'QUESTION') {
            this.consume();
            return { modifier: 'option', item: this.consume('IDENTIFIER').value };
        }
        return { modifier: 'none', item: this.consume('IDENTIFIER').value };
    }
}