// --- Types & Interfaces ---

type NodeType = 'Program' | 'BranchRule' | 'ProductRule' | 'BranchItem' | 'ProductItem';

interface ASTNode {
  type: string;
  id?: string;
  value?: any;
  label?: string;
  children?: ASTNode[];
}

// --- Lexer (字句解析) ---

class Lexer {
  private pos = 0;
  private tokens: string[] = [];

  constructor(input: string) {
    // 記号、クォート文字列、識別子を抽出する正規表現
    const regex = /"([^"\\]|\\.)*"|[a-zA-Z_]\w*|[:{}*?]|[\s]+/g;
    let match;
    while ((match = regex.exec(input)) !== null) {
      if (!match[0].trim()) continue; // 空白をスキップ
      this.tokens.push(match[0]);
    }
  }

  peek() { return this.tokens[this.pos]; }
  next() { return this.tokens[this.pos++]; }
  consume(expected?: string) {
    const token = this.next();
    if (expected && token !== expected) {
      throw new Error(`Expected "${expected}" but found "${token}" at position ${this.pos}`);
    }
    return token;
  }
  isEOF() { return this.pos >= this.tokens.length; }
}

// --- Parser (構文解析) ---

class Parser {
  private lexer: Lexer;

  constructor(input: string) {
    this.lexer = new Lexer(input);
  }

  public parse(): ASTNode {
    const rules: ASTNode[] = [];
    while (!this.lexer.isEOF()) {
      rules.push(this.parseRule());
    }
    return { type: 'Program', children: rules };
  }

  private parseRule(): ASTNode {
    const token = this.lexer.peek();
    if (token === 'branch') return this.parseBranchRule();
    if (token === 'product') return this.parseProductRule();
    throw new Error(`Unexpected token: ${token}. Expected "branch" or "product".`);
  }

  private parseBranchRule(): ASTNode {
    this.lexer.consume('branch');
    const id = this.lexer.next(); // identifier
    this.lexer.consume('{');
    const items: ASTNode[] = [];
    while (this.lexer.peek() !== '}') {
      const itemId = this.lexer.next();
      let label: string | undefined;
      if (this.lexer.peek() === 'with') {
        this.lexer.consume('with');
        label = this.lexer.next().replace(/"/g, ''); // string_literal
      }
      items.push({ type: 'BranchItem', id: itemId, label });
    }
    this.lexer.consume('}');
    return { type: 'BranchRule', id, children: items };
  }

  private parseProductRule(): ASTNode {
    this.lexer.consume('product');
    const id = this.lexer.next();
    this.lexer.consume('{');
    const items: ASTNode[] = [];
    while (this.lexer.peek() !== '}') {
      items.push(this.parseProductItem());
    }
    this.lexer.consume('}');
    return { type: 'ProductRule', id, children: items };
  }

  private parseProductItem(): ASTNode {
    let node: ASTNode;
    const token = this.lexer.peek();

    if (token.startsWith('"')) {
      // terminal
      node = { type: 'Terminal', value: this.lexer.next().replace(/"/g, '') };
    } else {
      // ( identifier : nonterminal )
      const id = this.lexer.next();
      this.lexer.consume(':');
      const nonTerminal = this.parseNonTerminal();
      node = { type: 'ProductItem', id, value: nonTerminal };
    }

    if (this.lexer.peek() === 'with') {
      this.lexer.consume('with');
      node.label = this.lexer.next().replace(/"/g, '');
    }
    return node;
  }

  private parseNonTerminal(): any {
    const token = this.lexer.peek();
    if (token === '*') {
      this.lexer.consume('*');
      return { modifier: 'repeat', id: this.lexer.next() };
    } else if (token === '?') {
      this.lexer.consume('?');
      return { modifier: 'option', id: this.lexer.next() };
    } else {
      return { modifier: 'none', id: this.lexer.next() };
    }
  }
}
