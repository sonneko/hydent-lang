// src/parser.rs

use crate::ast::*;
use crate::error::ParseError;
use crate::tokenizer::{Delimiter, Keyword, Operator, Token};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            position: 0,
            errors: Vec::new(),
        }
    }

    // --- エントリーポイント ---
    pub fn parse_program(&mut self) -> Result<Program<'a>, Vec<ParseError>> {
        let mut body = Vec::new();
        while !self.is_at_end() {
            match self.parse_declaration() {
                Ok(decl) => body.push(TopLevelItem::Declaration(decl)),
                Err(e) => {
                    self.errors.push(e);
                    // エラーから回復するため、次のセミコロンや宣言まで進む
                    self.synchronize();
                }
            }
        }

        if self.errors.is_empty() {
            Ok(Program { body })
        } else {
            Err(self.errors.clone()) // ここではcloneするが、所有権をムーブしても良い
        }
    }

    // --- 宣言のパース ---
    fn parse_declaration(&mut self) -> Result<Declaration<'a>, ParseError> {
        let is_public = self.match_token(Token::Keyword(Keyword::Pub));

        match self.current() {
            Token::Keyword(Keyword::Class) => self.parse_class_declaration(is_public),
            Token::Keyword(Keyword::Fn) => self.parse_function_declaration(is_public).map(Declaration::Function),
            // 他の宣言 (const, letなど) もここに追加
            _ => Err(ParseError::new("Expected a declaration (class, fn, etc.)")),
        }
    }

    fn parse_class_declaration(&mut self, is_public: bool) -> Result<Declaration<'a>, ParseError> {
        self.consume(Token::Keyword(Keyword::Class), "Expected 'class' keyword.")?;
        let name = self.consume_identifier("Expected class name.")?;
        self.consume(Token::Delimiter(Delimiter::LeftBrace), "Expected '{' after class name.")?;
        
        let mut members = Vec::new();
        while !self.check(Token::Delimiter(Delimiter::RightBrace)) && !self.is_at_end() {
            let is_member_public = self.match_token(Token::Keyword(Keyword::Pub));
            self.consume(Token::Keyword(Keyword::Fn), "Expected 'fn' for method.")?;
            let method = self.parse_function_declaration(is_member_public)?;
            members.push(ClassMember::Method(method));
        }

        self.consume(Token::Delimiter(Delimiter::RightBrace), "Expected '}' after class body.")?;
        
        Ok(Declaration::Class(ClassDeclaration { is_public, name, members }))
    }

    fn parse_function_declaration(&mut self, is_public: bool) -> Result<FunctionDeclaration<'a>, ParseError> {
        // "fn" は呼び出し元で消費済み
        let name = self.consume_identifier("Expected function name.")?;
        self.consume(Token::Delimiter(Delimiter::LeftParen), "Expected '(' after function name.")?;
        let params = self.parse_parameters()?;
        self.consume(Token::Delimiter(Delimiter::RightParen), "Expected ')' after parameters.")?;
        
        let return_type = if self.match_token(Token::Operator(Operator::Minus)) { // `->` を簡易的に `-` と `>` で
            self.consume(Token::Operator(Operator::Greater), "Expected '>' for arrow '->'")?;
            Some(Type { name: self.consume_identifier("Expected return type.")? })
        } else {
            None
        };

        let body = self.parse_block_expression()?;

        Ok(FunctionDeclaration { is_public, name, params, return_type, body })
    }

    fn parse_parameters(&mut self) -> Result<Vec<Param<'a>>, ParseError> {
        let mut params = Vec::new();
        if self.check(Token::Delimiter(Delimiter::RightParen)) {
            return Ok(params);
        }

        loop {
            let name = self.consume_identifier("Expected parameter name.")?;
            self.consume(Token::Delimiter(Delimiter::Colon), "Expected ':' after parameter name.")?;
            let param_type = Type { name: self.consume_identifier("Expected parameter type.")? };
            params.push(Param { name, param_type });
            
            if !self.match_token(Token::Delimiter(Delimiter::Comma)) {
                break;
            }
        }
        Ok(params)
    }

    // --- 文のパース ---
    fn parse_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        match self.current() {
            Token::Keyword(Keyword::Let) => self.parse_let_statement().map(Statement::Let),
            Token::Keyword(Keyword::Return) => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement<'a>, ParseError> {
        self.advance(); // let を消費
        let name = self.consume_identifier("Expected variable name.")?;
        self.consume(Token::Operator(Operator::Assign), "Expected '=' after variable name.")?;
        let initializer = self.parse_expression()?;
        self.consume(Token::Delimiter(Delimiter::Semicolon), "Expected ';' after let statement.")?;
        Ok(LetStatement { name, initializer })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement<'a>, ParseError> {
        self.advance(); // return を消費
        let value = self.parse_expression()?;
        self.consume(Token::Delimiter(Delimiter::Semicolon), "Expected ';' after return value.")?;
        Ok(ReturnStatement { value })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        let expr = self.parse_expression()?;
        self.consume(Token::Delimiter(Delimiter::Semicolon), "Expected ';' after expression.")?;
        Ok(Statement::Expression(expr))
    }

    // --- 式のパース (演算子優先順位を考慮) ---
    fn parse_expression(&mut self) -> Result<Expression<'a>, ParseError> {
        self.parse_term()
    }
    
    // 足し算・引き算 (低い優先順位)
    fn parse_term(&mut self) -> Result<Expression<'a>, ParseError> {
        let mut expr = self.parse_factor()?;
        while let Some(op) = self.match_any_operator(&[Operator::Plus, Operator::Minus]) {
            let right = self.parse_factor()?;
            expr = Expression::Binary { left: Box::new(expr), operator: op, right: Box::new(right) };
        }
        Ok(expr)
    }

    // 掛け算・割り算 (高い優先順位)
    fn parse_factor(&mut self) -> Result<Expression<'a>, ParseError> {
        // さらに高い優先順位のものをパース
        self.parse_primary()
    }

    // 基本的な式
    fn parse_primary(&mut self) -> Result<Expression<'a>, ParseError> {
        let token = self.advance();
        match token {
            Token::Literal(lit) => Ok(Expression::Literal(*lit)),
            Token::Identifier(id) => self.parse_identifier_expression(id),
            _ => Err(ParseError::new("Expected an expression.")),
        }
    }
    
    fn parse_identifier_expression(&mut self, id: &'a str) -> Result<Expression<'a>, ParseError> {
        // `::` が続く場合は静的アクセス
        if self.match_token(Token::Delimiter(Delimiter::ColonColon)) {
            let member = self.consume_identifier("Expected member name after '::'")?;
            // さらに `()` が続く場合は静的メソッド呼び出し
            if self.match_token(Token::Delimiter(Delimiter::LeftParen)) {
                // 引数のパースは省略
                self.consume(Token::Delimiter(Delimiter::RightParen), "Expected ')'")?;
                return Ok(Expression::Call { 
                    callee: Box::new(Expression::StaticAccess { class_name: id, member }),
                    args: vec![],
                });
            }
        }
        // それ以外はただの識別子
        Ok(Expression::Identifier(id))
    }


    fn parse_block_expression(&mut self) -> Result<BlockExpression<'a>, ParseError> {
        self.consume(Token::Delimiter(Delimiter::LeftBrace), "Expected '{' to start a block.")?;
        let mut statements = Vec::new();
        while !self.check(Token::Delimiter(Delimiter::RightBrace)) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        self.consume(Token::Delimiter(Delimiter::RightBrace), "Expected '}' to end a block.")?;
        Ok(BlockExpression { statements })
    }


    // --- ヘルパーメソッド ---
    fn consume(&mut self, expected: Token, message: &str) -> Result<&'a Token, ParseError> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(message))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<&'a str, ParseError> {
        match self.current() {
            Token::Identifier(id) => {
                self.advance();
                Ok(id)
            }
            _ => Err(ParseError::new(message)),
        }
    }

    fn match_token(&mut self, token_type: Token) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any_operator(&mut self, ops: &[Operator]) -> Option<Operator> {
        if let Token::Operator(op) = self.current() {
            if ops.contains(op) {
                self.advance();
                return Some(*op);
            }
        }
        None
    }

    fn check(&self, token_type: Token) -> bool {
        !self.is_at_end() && self.current() == &token_type
    }

    fn advance(&mut self) -> &'a Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Token::EndOfFile)
    }

    fn current(&self) -> &'a Token {
        self.tokens.get(self.position).unwrap_or(&Token::EndOfFile)
    }

    fn previous(&self) -> &'a Token {
        &self.tokens[self.position - 1]
    }
    
    fn synchronize(&mut self) {
        // パニックモードから回復するために、次の文の始まりまでトークンを捨てる
        self.advance();
        while !self.is_at_end() {
            if let Token::Delimiter(Delimiter::Semicolon) = self.previous() { return; }

            match self.current() {
                Token::Keyword(Keyword::Class) | Token::Keyword(Keyword::Fn) |
                Token::Keyword(Keyword::Let) | Token::Keyword(Keyword::Return) => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}