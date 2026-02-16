export type Brand<K, T> = T & { readonly __brand: K };

export type RustFunctionName = Brand<"RustFunctionName", string>;
export type RustASTTypeName = Brand<"RustASTTypeName", string>;
export type RustTokenTypeName = Brand<"RustTokenTypeName", string>;

export type IR = ParserFunction[];

export type ParserFunction = HookParserFunction | BranchParserFunction | ProductParserFunction;

export type HookParserFunction = {
    kind: "hook",
    functionName: RustFunctionName,
    astTypeName: RustASTTypeName,
};

export type BranchParserFunction = {
    kind: "branch",
    functionName: RustFunctionName,
    astTypeName: RustASTTypeName,
    branchesJudgebleInPeek0: {
        astTypeName: RustASTTypeName,
        firstTerminal: RustTokenTypeName,
    }[],
    branchesJudgebleInPeek1: {
        astTypeName: RustASTTypeName,
        firstTerminal: RustTokenTypeName,
        secondTerminal: RustTokenTypeName,
    }[],
    branchesNeedBacktrack: {
        astTypeName: RustASTTypeName,
        firstTerminal: RustTokenTypeName,
        secondTerminal: RustTokenTypeName,
    }[],
};

export type ProductParserFunction = {
    kind: "product",
    functionName: RustFunctionName,
    astTypeName: RustASTTypeName,
    elements: ({
        kind: "normal",
        astTypeName: RustASTTypeName,
    } | {
        kind: "boxed",
        astTypeName: RustASTTypeName,
    } | {
        kind: "repeat",
        astTypeName: RustASTTypeName,
    } | {
        kind: "option",
        astTypeName: RustASTTypeName,
    })[],
};

`
#[inline]
    fn parse_Module(&mut self) -> Result<Module, Self::Error> {
        let declarations = self.repeat(|this: &mut Self| this.parse_TopLevelDeclaration());
        Ok(Module { declarations })
    }

    #[inline]
    fn parse_TopLevelDeclaration(&mut self) -> Result<TopLevelDeclaration, Self::Error> {
        match self.peek_n::<0>() {
            Some(Token::Keyword(Keyword::Fn)) => {
                Ok(TopLevelDeclaration::Function(self.parse_Function()?))
            }
            _ => self.backtrack(|this| {
                if let Ok(val) = this.parse_Function() {
                    return Ok(TopLevelDeclaration::Function(val));
                }
                Err(Self::Error::create(
                    this.get_errors_arena(),
                    [],
                    this.peek_n::<0>(),
                ))
            }),
        }
    }

    #[inline]
    fn parse_Function(&mut self) -> Result<Function, Self::Error> {
        self.expect_token(Token::Keyword(Keyword::Fn))?;
        let name = self.parse_Identifier()?;
        self.expect_token(Token::Delimiter(Delimiter::LeftParen))?;
        let params = self.repeat(|this: &mut Self| this.parse_Parameter());
        self.expect_token(Token::Delimiter(Delimiter::RightParen))?;
        let return_type = self.parse_TypeLiteral().ok();
        let body = self.parse_BlockExpression()?;
        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }
`