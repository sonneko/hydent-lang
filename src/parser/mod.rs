pub struct Program(Vec<TopLevelDeclaration>);

pub enum TopLevelDeclaration {
    FunctionDeclaration(FunctionDeclaration),
    StaticDeclaration(StaticDeclaration),
    ClassDeclaration(ClassDeclaration),
    ProtocolDeclaration(ProtocolDeclaration),
    ImportDeclaration(ImportDeclaration),
    NamespaceDeclaration(NamespaceDeclaration),
}

pub struct FunctionDeclaration {
    pub access_modifier: IsPublic,
    pub name: Identifier,
    pub parameters: Vec<ParameterWithType>,
    pub return_type: Option<TypeLiteral>,
    pub body: BlockExpression,
}


pub struct VirtualFunctionDeclaration {
    pub access_modifier: IsPublic,
    pub name: Identifier,
    pub parameters: Vec<ParameterWithType>,
    pub return_type: Option<TypeLiteral>,
}

pub struct StaticDeclaration {
    pub access_modifier: IsPublic,
    pub name: Identifier,
    pub value: Expression,
}

pub struct ClassDeclaration {
    pub name: Identifier,
    pub members: Vec<ClassMember>,
    pub functions: Vec<FunctionDeclaration>,
}

pub struct ProtocolDeclaration {
    pub name: Identifier,
    pub functions: Vec<FunctionDeclaration>,
    pub virtual_functioinis: Vec<VirtualFunctionDeclaration>,
}

pub struct ImportDeclaration {
    pub path: String,
    pub alias: Vec<Identifier>,
}

pub struct NamespaceDeclaration {
    pub name: Identifier,
    pub content: Program,
}


pub enum ClassMember {
    StaticDeclaration(StaticDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}

pub struct IsPublic(bool);

pub struct Identifier(String);

pub enum Statement {
    LetVariableStatement(ValiableStatement),
    ConstValiableStatement(ValiableStatement),
    ExpressionStatement(ExpressionStatement),
    ReturnStatement(Expression),
    BreakStatement(Expression),
    ContinueStatement(Expression),
}

pub struct ValiableStatement {
    name: Identifier,
    value: Option<Expression>,
}

pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    BlockExpression(BlockExpression),
    IfExpression(IfExpression),
    ReturnExpression(ReturnExpression),
    WhileExpression(WhileExpression),
    ForExpression(ForExpression),
    MatchExpression(MatchExpression),
    FunctionCall(FunctionCall),
    PropertyAccess()
}

pub enum TypeLiteral {
    Identifier(Identifier),
    Array(Box<TypeLiteral>),
    Tuple(Vec<TypeLiteral>),
    Dictionary(Box<TypeLiteral>, Box<TypeLiteral>),
    Union(Vec<TypeLiteral>),
    Intersection(Vec<TypeLiteral>),
}
