 // ```
// <bnf> ::= { <rule> }
// 
// <rule> ::= <branch_rule> | <product_rule>
// 
// <branch_rule> ::= "branch" <identifier> "{" <branch_rule_inner> "}"
// <branch_rule_inner> ::= { <identifier> [ "with" <string_literal> ] }
// 
// <product_rule> ::= "product" <identifier> "{" <product_inner> "}"
// <product_inner> ::= { <product_item> [ "with" <string_literal> ] }
// <product_item> ::= ( <identifier> ":" <nonterminal> ) | <terminal>
// <nonterminal> ::= <repete_item> | <option_item> | <item>
// <repete_item> ::= "*" <item>
// <option_item> ::= "?" <item>
// <item> ::= <identifier>
// <terminal> ::= <string_literal>
// ```

// --- AST Types ---

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

const TOKEN_PATTERNS: Array<{ kind: TokenKind; regex: RegExp }> = [
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

export class Lexer {
  private input: string;
  private position: number;

  constructor(input: string) {
    this.input = input;
    this.position = 0;
  }

  public tokenize(): Token[] {
    const tokens: Token[] = [];

    while (this.position < this.input.length) {
      // skip whitespace
      const whitespace = this.input.slice(this.position).match(/^\s+/);
      if (whitespace) {
        this.position += whitespace[0].length;
        continue;
      }

      // skip comment
      const comment = this.input.slice(this.position).match(/^\/\/.*\n/);
      if (comment) {
        this.position += comment[0].length;
        continue;
      }

      let matched = false;
      const currentSlice = this.input.slice(this.position);

      for (const { kind, regex } of TOKEN_PATTERNS) {
        const match = currentSlice.match(regex);
        if (match) {
          matched = true;
          this.position += match[0].length;

          if (kind === 'StringLiteral') {
            tokens.push({ kind, value: match[1] });
          } else if (kind === 'Identifier') {
            tokens.push({ kind, value: match[0] });
          } else {
            switch (kind) {
              case 'Branch': tokens.push({ kind: 'Branch' }); break;
              case 'Product': tokens.push({ kind: 'Product' }); break;
              case 'With': tokens.push({ kind: 'With' }); break;
              case 'LBrace': tokens.push({ kind: 'LBrace' }); break;
              case 'RBrace': tokens.push({ kind: 'RBrace' }); break;
              case 'Colon': tokens.push({ kind: 'Colon' }); break;
              case 'Star': tokens.push({ kind: 'Star' }); break;
              case 'Question': tokens.push({ kind: 'Question' }); break;
              default: break;
            }
          }
          break;
        }
      }

      if (!matched) {
        throw new Error(`Unexpected character at position ${this.position}: "${this.input[this.position]}"`);
      }
    }

    tokens.push({ kind: 'EOF' });
    return tokens;
  }
}

// --- Parser Implementation ---

export class Parser {
  private tokens: Token[];
  private position: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.position = 0;
  }

  private peek(): Token {
    return this.tokens[this.position];
  }

  private advance(): Token {
    const token = this.tokens[this.position];
    this.position++;
    return token;
  }

  private consumeIdentifier(): string {
      const token = this.peek();
      if (token.kind === 'Identifier') {
          this.advance();
          return token.value;
      }
      throw new Error(`Expected Identifier, but got ${token.kind}`);
  }

  private consumeStringLiteral(): string {
      const token = this.peek();
      if (token.kind === 'StringLiteral') {
          this.advance();
          return token.value;
      }
      throw new Error(`Expected StringLiteral, but got ${token.kind}`);
  }
  
  private consumeKeyword(kind: 'Branch' | 'Product' | 'LBrace' | 'RBrace' | 'Colon' | 'With' | 'Star' | 'Question'): void {
      const token = this.peek();
      if (token.kind === kind) {
          this.advance();
          return;
      }
      throw new Error(`Expected ${kind}, but got ${token.kind}`);
  }

  public parse(): Grammar {
    const rules: Rule[] = [];
    while (this.peek().kind !== 'EOF') {
      rules.push(this.parseRule());
    }
    return rules;
  }

  private parseRule(): Rule {
    const token = this.peek();
    if (token.kind === 'Branch') {
      return this.parseBranchRule();
    } else if (token.kind === 'Product') {
      return this.parseProductRule();
    } else {
      throw new Error(`Expected 'branch' or 'product', but got ${token.kind}`);
    }
  }

  // <branch_rule> ::= "branch" <identifier> "{" <branch_rule_inner> "}"
  private parseBranchRule(): BranchRule {
    this.consumeKeyword('Branch');
    const name = this.consumeIdentifier();
    this.consumeKeyword('LBrace');

    const variants: BranchVariant[] = [];
    // <branch_rule_inner> ::= { <identifier> [ "with" <string_literal> ] }
    while (this.peek().kind !== 'RBrace' && this.peek().kind !== 'EOF') {
      const variantName = this.consumeIdentifier();
      let note: string | null = null;
      
      if (this.peek().kind === 'With') {
        this.consumeKeyword('With');
        note = this.consumeStringLiteral();
      }
      
      variants.push({ name: variantName, note });
    }

    this.consumeKeyword('RBrace');
    return { kind: 'Branch', name, variants };
  }

  // <product_rule> ::= "product" <identifier> "{" <product_inner> "}"
  private parseProductRule(): ProductRule {
    this.consumeKeyword('Product');
    const name = this.consumeIdentifier();
    this.consumeKeyword('LBrace');

    const members: ProductMember[] = [];
    
    // <product_inner> ::= { <product_item> [ "with" <string_literal> ] }
    while (this.peek().kind !== 'RBrace' && this.peek().kind !== 'EOF') {
        const member = this.parseProductItem();
        
        let note: string | null = null;
        if (this.peek().kind === 'With') {
            this.consumeKeyword('With');
            note = this.consumeStringLiteral();
        }
        
        if (member.kind === 'Field') {
            members.push({ ...member, note });
        } else {
            members.push({ ...member, note });
        }
    }

    this.consumeKeyword('RBrace');
    return { kind: 'Product', name, members };
  }

  // <product_item> ::= ( <identifier> ":" <nonterminal> ) | <terminal>
  private parseProductItem(): ProductMember {
    const token = this.peek();

    if (token.kind === 'StringLiteral') {
        // <terminal>
        const value = this.consumeStringLiteral();
        return { kind: 'Terminal', value, note: null };
    } else if (token.kind === 'Identifier') {
        // <identifier> ":" <nonterminal>
        const name = this.consumeIdentifier();
        this.consumeKeyword('Colon');
        const typeRef = this.parseNonTerminal();
        return { kind: 'Field', name, type: typeRef, note: null };
    } else {
        throw new Error(`Expected Identifier or StringLiteral in product item, got ${token.kind}`);
    }
  }

  // <nonterminal> ::= <repete_item> | <option_item> | <item>
  private parseNonTerminal(): TypeReference {
    const token = this.peek();
    
    if (token.kind === 'Star') {
        // <repete_item> ::= "*" <item>
        this.consumeKeyword('Star');
        const name = this.consumeIdentifier();
        return { name, modifier: 'List' };
    } else if (token.kind === 'Question') {
        // <option_item> ::= "?" <item>
        this.consumeKeyword('Question');
        const name = this.consumeIdentifier();
        return { name, modifier: 'Option' };
    } else {
        // <item> ::= <identifier>
        const name = this.consumeIdentifier();
        return { name, modifier: 'None' };
    }
  }
}

export function parse(src: string): Grammar {
  const lexer = new Lexer(src);
  const tokens = lexer.tokenize();
  const parser = new Parser(tokens);
  return parser.parse()
}
