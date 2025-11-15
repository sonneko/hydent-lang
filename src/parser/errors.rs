// src/ast.rs

// 型エイリアスで見やすくする
type BExpr = Box<Expression>;

#[derive(Debug)]
pub struct Program<'a> {
    pub body: Vec<TopLevelItem<'a>>,
}

#[derive(Debug)]
pub enum TopLevelItem<'a> {
    Declaration(Declaration<'a>),
    // Import, Namespace などもここに追加
}

#[derive(Debug)]
pub enum Declaration<'a> {
    Function(FunctionDeclaration<'a>),
    Class(ClassDeclaration<'a>),
    Const(ConstDeclaration<'a>),
    Let(LetStatement<'a>),
}

#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    pub is_public: bool,
    pub name: &'a str,
    pub params: Vec<Param<'a>>,
    pub return_type: Option<Type<'a>>,
    pub body: BlockExpression<'a>,
}

#[derive(Debug)]
pub struct ClassDeclaration<'a> {
    pub is_public: bool,
    pub name: &'a str,
    pub members: Vec<ClassMember<'a>>,
}

#[derive(Debug)]
pub enum ClassMember<'a> {
    Method(FunctionDeclaration<'a>),
    // Field, StaticConst など
}

#[derive(Debug)]
pub struct ConstDeclaration<'a> {
    // ...
}

#[derive(Debug)]
pub struct Param<'a> {
    pub name: &'a str,
    pub param_type: Type<'a>,
}

#[derive(Debug)]
pub struct Type<'a> {
    pub name: &'a str,
    // ジェネリクスなども将来的に追加
}

#[derive(Debug)]
pub enum Statement<'a> {
    Let(LetStatement<'a>),
    Return(ReturnStatement<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub struct LetStatement<'a> {
    pub name: &'a str,
    pub initializer: Expression<'a>,
}

#[derive(Debug)]
pub struct ReturnStatement<'a> {
    pub value: Expression<'a>,
}


#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Literal(super::tokenizer::Literal<'a>),
    Binary {
        left: BExpr,
        operator: super::tokenizer::Operator,
        right: BExpr,
    },
    Call {
        callee: BExpr,
        args: Vec<Expression<'a>>,
    },
    StaticAccess {
        class_name: &'a str,
        member: &'a str,
    },
    Block(BlockExpression<'a>),
    If {
        condition: BExpr,
        consequence: Box<BlockExpression<'a>>,
        alternative: Option<Box<BlockExpression<'a>>>,
    },
}

#[derive(Debug)]
pub struct BlockExpression<'a> {
    pub statements: Vec<Statement<'a>>,
}