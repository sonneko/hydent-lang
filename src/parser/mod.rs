pub mod errors;
pub mod parser;

use crate::compiler::{span::Span, symbol::Symbol};
use crate::compiler::arena::{ArenaBox, ArenaIter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Identifier(pub Symbol);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StringLiteral(pub Span);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CharLiteral(pub char);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumLiteral {
    Integer(i32),
    Float(f32),
    DoubleInteger(i64),
    DoubleFloat(f64),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BoolLiteral {
    True,
    False,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsExtern {
    Extern,
    NotExtern,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RelationalOperator {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShiftOperator {
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CanPanics {
    CanPanics,
    CannotPanics,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MultiplicativeOperator {
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrefixOperator {
    BitNot,
    Not,
    Minus,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsTry {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsIgnore {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsAsync {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FieldDeclarationKeyword {
    Final,
    Mut,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsMut {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct DocsComments {
    pub comments: ArenaIter<Span>,
    pub annotation: ArenaIter<Annotation>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    RemainderAssign,
    PowerAssign,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ast {
    pub top_level: TopLevel
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TopLevel {
	pub children: ArenaIter<TopLevelStatement>
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TopLevelStatement {
    ImportDeclaration(ImportDeclaration),
    StaticVariableDeclaration(StaticVariableDeclaration),
    ClassDeclaration(ClassDeclaration),
    EnumDeclaration(EnumDeclaration),
    StructDeclaration(StructDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ProtocolDeclaration(ProtocolDeclaration),
    ModuleDeclaration(ModuleDeclaration),
    Annotation(Annotation),
    TypeAliasDeclaration(TypeAliasDeclaration),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ImportDeclaration {
	ImportSpecific(ImportSpecific, Span),
    ImportAllAs(ImportAllAs, Span),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ImportSpecific(pub IdentifierList);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ImportAllAs(pub Identifier);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StaticVariableDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ClassDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics,
    pub implements_protocol: ImplementsProtocol,
    pub function_declarations: ArenaIter<FunctionDeclaration>,
    pub field_declarations: ArenaIter<FieldDeclaration>,
    pub type_alias_declarations: ArenaIter<TypeAliasDeclaration>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EnumDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics,
    pub implements_protocol: ImplementsProtocol,
    pub enum_members: ArenaIter<EnumMember>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EnumMember {
    EnumVariant(EnumVariant),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EnumVariant(pub Option<TypeLiteralList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub struct_body: StructBody,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StructBody {
	StructBlockBody(StructBlockBody),
	StructTupleBody(StructTupleBody),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructBlockBody (pub ArenaIter<FieldDeclaration>);


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructTupleBody(pub TypeLiteralList);


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FunctionDeclaration {
	pub docs_comments: DocsComments,
    pub is_extern: IsExtern,
    pub is_public: IsPublic,
    pub is_async: IsAsync,
    pub identifier: Identifier,
    pub generics: Generics,
    pub params_with_types: ParamsWithTypes,
    pub return_type_literal: Option<TypeLiteral>,
    pub can_panics: CanPanics,
    pub block_expression: Option<BlockExpression>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ProtocolDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics,
    pub implements_protocol: ImplementsProtocol,
    pub protocol_members: ArenaIter<ProtocolMember>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ProtocolMember {
	FunctionDeclaration(FunctionDeclaration),
    TypeAliasDeclaration(TypeAliasDeclaration),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ModuleDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub top_level: TopLevel,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Annotation {
    pub identifier: Identifier,
    pub literals: ArenaIter<Literal>,	
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TypeAliasDeclaration {
	pub docs_comments: DocsComments,
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics,
    pub type_literal: TypeLiteral,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IfExpression {
    pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ElseIfClause {
    pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ElseClause {
	pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MatchExpression {
    pub expression: Expression,
    pub match_arms: ArenaIter<MatchArm>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard_if: Option<IfExpression>,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LoopExpression {
	pub loop_times: Option<Expression>,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WhileExpression {
	pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ForStatement {
	pub pattern: Pattern,
    pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ForExpression {
	pub expression: Expression,
    pub pipeline_arms: ArenaIter<PipelineArm>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PipelineArm {
	pub pattern: Pattern,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IfLetExpression {
	pub pattern: Pattern,
    pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WhileLetExpression {
	pub pattern: Pattern,
    pub expression: Expression,
    pub block_expression: BlockExpression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PipeExpression {
	pub expression: Expression,
    pub pipe_arms: ArenaIter<PipeArm>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PipeArm {
    pub pattern: Pattern,
	pub guard_if: Expression,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Closer {
	pub closer_params: Option<CloserParams>,
    pub block_expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CloserParams(pub ArenaIter<CloserParamItem>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CloserParamItem {
	ParamWithType(ParamWithType),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Accesser(pub ArenaIter<Identifier>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Params(pub Option<ExpressionList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Expression(pub LogicalOrExpr);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LogicalOrExpr(pub ArenaIter<LogicalAndExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LogicalAndExpr(pub ArenaIter<BitwiseOrExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BitwiseOrExpr(pub ArenaIter<BitwiseXorExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BitwiseXorExpr(pub ArenaIter<BitwiseAndExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BitwiseAndExpr(pub ArenaIter<EqualityExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EqualityExpr {
	pub left: RelationalExpr,
    pub operator: EqualityOperator,
    pub right: RelationalExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RelationalExpr {
    pub left: ShiftExpr,
    pub operator: RelationalOperator,
    pub right: ShiftExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ShiftExpr {
	pub left: AdditiveExpr,
    pub operator: ShiftOperator,
    pub right: AdditiveExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AdditiveExpr {
    pub left: MultiplicativeExpr,
    pub oprator: AdditiveOperator,
    pub right: MultiplicativeExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MultiplicativeExpr {
	pub left: PowerExpr,
    pub operator: MultiplicativeOperator,
    pub right: PowerExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PowerExpr(pub ArenaIter<PrefixExpr>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PrefixExpr {
	pub operators: ArenaIter<PrefixOperator>,
    pub expression: PrimaryExpr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PrimaryExpr {
	BlockExpression(BlockExpression),
    IfExpression(IfExpression),
    MatchExpression(MatchExpression),
    LoopExpression(LoopExpression),
    WhileExpression(WhileExpression),
    ForExpression(ForExpression),
    PipeExpression(PipeExpression),
    Accesser(Accesser),
    Literal(Literal),
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    FieldAccess(FieldAccess),
    AwaitExpression(Expression),
    TupleOrGroupedExpression(TupleOrGroupedExpression),
    StructLiteral(StructLiteral),
    Closer(Closer),
    IfLetExpression(IfLetExpression),
    WhileLetExpression(WhileLetExpression),
    ArrayLiteral(ArrayLiteral),
    IndexAccess(IndexAccess),
    CastExpression(CastExpression),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FunctionCall {
	pub is_try: IsTry,
    pub accesser: Accesser,
    pub params: Params,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MethodCall {
	pub is_try: IsTry,
    pub accesser: Accesser,
    pub identifier: Identifier,
    pub params: Params,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldAccess {
	pub accesser: Accesser,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TupleOrGroupedExpression(pub Option<ExpressionList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructLiteral {
	pub accesser: Accesser,
    pub struct_literal_fields: Option<StructLiteralFields>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructLiteralFields(pub ArenaIter<StructFieldInit>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructFieldInit {
	pub field_name: Identifier,
    pub expression: Option<Expression>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ArrayLiteral(pub Option<ExpressionList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IndexAccess {
	pub container: Expression,
    pub index: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CastExpression {
	pub target: Expression,
    pub type_literal: TypeLiteral,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Statement {
	IfExpression(IfExpression),
    MatchExpression(MatchExpression),
    LoopExpression(LoopExpression),
    WhileExpression(WhileExpression),
    ForStatement(ForStatement),
    ExpressionStatement(ExpressionStatement),
    VariableDeclaration(VariableDeclaration),
    ReturnStatement(Expression),
    BreakStatement(Expression),
    ContinueStatement,
    AssignmentStatement(AssignmentStatement),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ExpressionStatement {
	pub is_ignore: IsIgnore,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VariableDeclaration {
	pub variable_declaration_keyword: VariableDeclarationKeyword,
    pub pattern: Pattern,
    pub type_literal: Option<TypeLiteral>,
    pub variable_declaration_assignment: VariableDeclarationAssignment,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VariableDeclarationKeyword {
	Let,
    Const,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VariableDeclarationAssignment {
	pub assignment_operator: AssignmentOperator,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AssignmentStatement {
	pub accesser: Accesser,
    pub assignment_operator: AssignmentOperator,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldDeclaration {
    pub field_declaration_keyword: FieldDeclarationKeyword,
	pub identifier: Identifier,
    pub type_literal: TypeLiteral,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParamWithType {
	Param {
        is_mut: IsMut,
        identifier: Identifier,
        type_literal: TypeLiteral,
        default_value: Option<Expression>,
    },
    This {
        is_mut: IsMut,
    },
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ParamsWithTypes(pub Option<ParamWithType>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ParamWithTypesList(pub ArenaIter<ParamsWithTypes>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BlockExpression {
    StatementList(ArenaIter<Statement>),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum IsPublic {
	Public,
    Private,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Literal {
	StringLiteral(StringLiteral),
    CharLiteral(CharLiteral),
    NumLiteral(NumLiteral),
    BoolLiteral(BoolLiteral),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TypeLiteral {
	Identifier {
        accesser: Accesser,
        generics: Option<Generics>,
    },
    ImplType(ArenaBox<TypeLiteral>),
    TupleType(TupleType),
    TypeOf(Expression),
    Bool,
    Int,
    DoubleInt,
    Float,
    DoubleFloat,
    Char,
    Usize,
    Any,
    Never,
    Void,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GenericTypeArgs(pub TypeLiteralList);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TupleType(pub Option<TypeLiteralList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pattern {
	Identifier(Identifier),
    Wild,
    TupleStructPattern(TupleStructPattern),
    TuplePattern(TuplePattern),
    StructPattern(StructPattern),
    Accesser(Accesser),
    Literal(Literal),
    RangePattern(RangePattern),
    BindingPattern(BindingPattern),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TupleStructPattern {
	pub accesser: Accesser,
    pub pattern_list: Option<PatternList>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TuplePattern(pub Option<PatternList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructPattern {
	pub accesser: Accesser,
    pub struct_pattern_fields: Option<StructPatternFields>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructPatternFields(pub ArenaIter<StructPatternField>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StructPatternField {
	pub identifier: Identifier,
    pub pattern: Option<Pattern>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RangePattern {
	Num {
        left: NumLiteral,
        operator: RangeOp,
        right: NumLiteral,
    },
    Char {
        left: CharLiteral,
        operator: RangeOp,
        right: CharLiteral,
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RangeOp {
	InRange,
    InRangeOr,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BindingPattern {
	pub identifier: Identifier,
    pub pattern: ArenaBox<Pattern>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Generics(pub GenericParamDefList);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GenericParamDefList(pub ArenaIter<GenericParamDef>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GenericParamDef {
	pub identifier: Identifier,
    pub generic_bound: Option<GenericBound>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GenericBound(pub ArenaIter<TypeLiteral>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ImplementsProtocol(pub Option<AccesserList>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IdentifierList(pub ArenaIter<Identifier>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TypeLiteralList(pub ArenaIter<TypeLiteral>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ExpressionList(pub ArenaIter<Expression>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PatternList(pub ArenaIter<Pattern>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AccesserList(pub ArenaIter<Accesser>);
