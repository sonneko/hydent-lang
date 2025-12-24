import { BNF_Alternative, BNF_AST, BNF_Factor, BNF_Rule, BNF_Term, Token, TokenType, } from "./type";


class BNF_Lexer {
    private src: string;
    private pos: number = 0;
    private line: number = 1;
    private col: number = 1;

    constructor(src: string) {
        this.src = src;
    }

    private peek(): string {
        return this.pos < this.src.length ? this.src[this.pos] : "";
    }

    private advance(): string {
        const char = this.peek();
        if (char === "\n") {
            this.line++;
            this.col = 1;
        } else {
            this.col++;
        }
        this.pos++;
        return char;
    }

    private match(str: string): boolean {
        if (this.src.startsWith(str, this.pos)) {
            for (let i = 0; i < str.length; i++) this.advance();
            return true;
        }
        return false;
    }

    private skipWhitespaceAndComments() {
        while (true) {
            const char = this.peek();
            if (/\s/.test(char)) {
                this.advance();
            } else if (this.src.startsWith("//", this.pos)) {
                // Line comment
                while (this.peek() && this.peek() !== "\n") {
                    this.advance();
                }
            } else if (this.src.startsWith("/*", this.pos)) {
                // Block comment
                this.advance(); this.advance(); // consume /*
                while (this.pos < this.src.length && !this.src.startsWith("*/", this.pos)) {
                    this.advance();
                }
                if (this.src.startsWith("*/", this.pos)) {
                    this.advance(); this.advance(); // consume */
                }
            } else {
                break;
            }
        }
    }

    public tokenize(): Token[] {
        const tokens: Token[] = [];

        while (this.pos < this.src.length) {
            this.skipWhitespaceAndComments();
            if (this.pos >= this.src.length) break;

            const char = this.peek();
            const startLine = this.line;
            const startCol = this.col;

            // ::=
            if (this.match("::=")) {
                tokens.push({ type: "ASSIGN", value: "::=", line: startLine, col: startCol });
                continue;
            }

            // Single char tokens
            if (char === "|") { tokens.push({ type: "PIPE", value: "|", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === "[") { tokens.push({ type: "LBRACKET", value: "[", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === "]") { tokens.push({ type: "RBRACKET", value: "]", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === "{") { tokens.push({ type: "LBRACE", value: "{", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === "}") { tokens.push({ type: "RBRACE", value: "}", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === "(") { tokens.push({ type: "LPAREN", value: "(", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === ")") { tokens.push({ type: "RPAREN", value: ")", line: startLine, col: startCol }); this.advance(); continue; }
            if (char === ";") { tokens.push({ type: "SEMICOLON", value: ";", line: startLine, col: startCol }); this.advance(); continue; }

            // Terminal String "..."
            if (char === '"') {
                this.advance(); // skip opening "
                let value = "";
                while (this.pos < this.src.length) {
                    const c = this.peek();
                    if (c === '"') {
                        break; // closing "
                    }
                    if (c === '\\') {
                        this.advance(); // skip backslash
                        value += this.advance(); // take escaped char
                    } else {
                        value += this.advance();
                    }
                }
                if (this.peek() === '"') {
                    this.advance(); // skip closing "
                    tokens.push({ type: "TERMINAL", value, line: startLine, col: startCol });
                    continue;
                } else {
                    throw new Error(`Unterminated string literal at ${startLine}:${startCol}`);
                }
            }

            // Non-Terminal Character <...>
            if (char === '<') {
                // We capture everything until >
                // Note: The example uses <IDENTIFIER> so simple parsing works.
                // If nested brackets were allowed, this would need to be smarter, but standard BNF doesn't nest <> for names.
                let value = "";
                this.advance(); // skip <
                while (this.pos < this.src.length && this.peek() !== '>') {
                    value += this.advance();
                }
                if (this.peek() === '>') {
                    this.advance(); // skip >
                    tokens.push({ type: "NON_TERMINAL", value, line: startLine, col: startCol });
                    continue;
                } else {
                    throw new Error(`Unterminated non-terminal at ${startLine}:${startCol}`);
                }
            }

            throw new Error(`Unexpected character '${char}' at ${startLine}:${startCol}`);
        }

        tokens.push({ type: "EOF", value: "", line: this.line, col: this.col });
        return tokens;
    }
}


class BNF_Parser {
    private tokens: Token[] = [];
    private pos: number = 0;

    public parse(source: string): BNF_AST {
        const lexer = new BNF_Lexer(source);
        this.tokens = lexer.tokenize();
        this.pos = 0;

        const rules: BNF_Rule[] = [];

        while (this.peek().type !== "EOF") {
            rules.push(this.parseRule());
        }

        return { rules };
    }

    private peek(): Token {
        return this.tokens[this.pos];
    }

    private advance(): Token {
        if (this.pos < this.tokens.length) {
            return this.tokens[this.pos++];
        }
        return this.tokens[this.tokens.length - 1];
    }

    private match(type: TokenType): boolean {
        if (this.peek().type === type) {
            this.advance();
            return true;
        }
        return false;
    }

    private expect(type: TokenType, errorMessage?: string): Token {
        if (this.peek().type === type) {
            return this.advance();
        }
        throw new Error(
            errorMessage ||
            `Expected ${type} but got ${this.peek().type} ('${this.peek().value}') at line ${this.peek().line}`
        );
    }

    // <rule> ::= <non_terminal> "::=" <alternative> [";"]
    private parseRule(): BNF_Rule {
        const nameToken = this.expect("NON_TERMINAL", "Expected rule name (e.g. <rule>)");
        this.expect("ASSIGN", "Expected '::=' after rule name");

        const alternative = this.parseAlternative();

        // Optional semicolon handling (as seen in the provided example)
        this.match("SEMICOLON");

        return {
            name: nameToken.value, // Removed < and > during lexing
            alternative
        };
    }

    // <alternative> ::= <term> { "|" <term> }
    private parseAlternative(): BNF_Alternative {
        const terms: BNF_Term[] = [];

        terms.push(this.parseTerm());

        while (this.match("PIPE")) {
            terms.push(this.parseTerm());
        }

        return { terms };
    }

    // <term> ::= <factor> { <factor> }
    private parseTerm(): BNF_Term {
        const factors: BNF_Factor[] = [];

        // Loop while the current token can be the start of a factor
        while (this.isFactorStart()) {
            // Edge case check: In standard BNF, rules follow each other.
            // If we see <ID> "::=", that is the start of a NEW rule, not a factor of the current term.
            // We need 2-token lookahead to distinguish "<factor>" from "<next_rule_name> ::="
            if (this.peek().type === "NON_TERMINAL") {
                if (this.lookahead(1).type === "ASSIGN") {
                    break; // Start of next rule, stop parsing term
                }
            }

            factors.push(this.parseFactor());
        }

        return { factors };
    }

    private isFactorStart(): boolean {
        const type = this.peek().type;
        return (
            type === "NON_TERMINAL" ||
            type === "TERMINAL" ||
            type === "LBRACKET" || // [
            type === "LBRACE" ||   // {
            type === "LPAREN"      // (
        );
    }

    private lookahead(offset: number): Token {
        if (this.pos + offset < this.tokens.length) {
            return this.tokens[this.pos + offset];
        }
        return this.tokens[this.tokens.length - 1];
    }

    // <factor>
    private parseFactor(): BNF_Factor {
        const token = this.peek();

        if (token.type === "NON_TERMINAL") {
            this.advance();
            return {
                node_type: "character",
                character: token.value
            };
        } else if (token.type === "TERMINAL") {
            this.advance();
            return {
                node_type: "terminal_string",
                terminal_string: token.value
            };
        } else if (this.match("LBRACKET")) { // [ ... ] -> Option
            const alt = this.parseAlternative();
            this.expect("RBRACKET", "Expected ']'");
            return {
                node_type: "option",
                alternative: alt
            };
        } else if (this.match("LBRACE")) { // { ... } -> Repeat
            const alt = this.parseAlternative();
            this.expect("RBRACE", "Expected '}'");
            return {
                node_type: "repeat",
                alternative: alt
            };
        } else if (this.match("LPAREN")) { // ( ... ) -> Group
            const alt = this.parseAlternative();
            this.expect("RPAREN", "Expected ')'");
            return {
                node_type: "group",
                alternative: alt
            };
        }

        throw new Error(`Unexpected token in factor: ${token.type} (${token.value}) at line ${token.line}`);
    }
}

/**
 * parse bnf string and return ast
 * @param bnf parse target bnf string
 * @returns result
 */
export function parseBNF(bnf: string): BNF_AST | null {
    const parser = new BNF_Parser();
    try {
        return parser.parse(bnf);
    } catch (err) {
        return null;
    }
}
