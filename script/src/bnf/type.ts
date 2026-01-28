// --- トークンの定義 ---
type TokenType = 
  | 'KEYWORD_BRANCH' | 'KEYWORD_PRODUCT' | 'KEYWORD_WITH'
  | 'LBRACE' | 'RBRACE' | 'COLON' | 'STAR' | 'QUESTION'
  | 'IDENTIFIER' | 'STRING_LITERAL' | 'EOF';

interface Token {
  type: TokenType;
  value: string;
}

