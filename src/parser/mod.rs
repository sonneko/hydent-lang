pub mod errors;
pub mod parser;

use crate::common::{span::Span, symbol::Symbol};


#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub Symbol);

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral(pub Span);

#[derive(Debug, PartialEq, Clone)]
pub struct CharLiteral(pub char);

#[derive(Debug, PartialEq, Clone)]
pub enum NumLiteral {
    Integer(i32),
    Float(f32),
    DoubleInteger(i64),
    DoubleFloat(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoolLiteral {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsExtern {
    Extern,
    NotExtern,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RelationalOperator {
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShiftOperator {
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CanPanics {
    CanPanics,
    CannotPanics,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MultiplicativeOperator {
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOperator {
    BitNot,
    Not,
    Minus,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsTry {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsIgnore {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsAsync {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldDeclarationKeyword {
    Final,
    Mut,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsMut {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DocsComments<'ast> {
    pub comments: &'ast [Span],
    pub annotation: &'ast [Annotation<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    RemainderAssign,
    PowerAssign,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Ast<'ast> {
    pub top_level: TopLevel<'ast>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TopLevel<'ast> {
	pub children: &'ast [TopLevelStatement<'ast>]
}

#[derive(Debug, PartialEq, Clone)]
pub enum TopLevelStatement<'ast> {
    ImportDeclaration(ImportDeclaration<'ast>),
    StaticVariableDeclaration(StaticVariableDeclaration<'ast>),
    ClassDeclaration(ClassDeclaration<'ast>),
    EnumDeclaration(EnumDeclaration<'ast>),
    StructDeclaration(StructDeclaration<'ast>),
    FunctionDeclaration(FunctionDeclaration<'ast>),
    ProtocolDeclaration(ProtocolDeclaration<'ast>),
    ModuleDeclaration(ModuleDeclaration<'ast>),
    Annotation(Annotation<'ast>),
    TypeAliasDeclaration(TypeAliasDeclaration<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportDeclaration<'ast> {
	ImportSpecific(ImportSpecific<'ast>, Span),
    ImportAllAs(ImportAllAs, Span),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportSpecific<'ast>(pub IdentifierList<'ast>);

#[derive(Debug, PartialEq, Clone)]
pub struct ImportAllAs(pub Identifier);

#[derive(Debug, PartialEq, Clone)]
pub struct StaticVariableDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics<'ast>,
    pub implements_protocol: ImplementsProtocol<'ast>,
    pub function_declarations: &'ast [FunctionDeclaration<'ast>],
    pub field_declarations: &'ast [FieldDeclaration<'ast>],
    pub type_alias_declarations: &'ast [TypeAliasDeclaration<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics<'ast>,
    pub implements_protocol: ImplementsProtocol<'ast>,
    pub enum_members: &'ast [EnumMember<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumMember<'ast> {
    EnumVariant(EnumVariant<'ast>),
    FunctionDeclaration(FunctionDeclaration<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant<'ast>(pub Option<TypeLiteralList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub struct_body: StructBody<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructBody <'ast>{
	StructBlockBody(StructBlockBody<'ast>),
	StructTupleBody(StructTupleBody<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructBlockBody<'ast> (pub &'ast [FieldDeclaration<'ast>]);


#[derive(Debug, PartialEq, Clone)]
pub struct StructTupleBody<'ast>(pub TypeLiteralList<'ast>);


#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_extern: IsExtern,
    pub is_public: IsPublic,
    pub is_async: IsAsync,
    pub identifier: Identifier,
    pub generics: Generics<'ast>,
    pub params_with_types: ParamsWithTypes<'ast>,
    pub return_type_literal: Option<TypeLiteral<'ast>>,
    pub can_panics: CanPanics,
    pub block_expression: Option<BlockExpression<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics<'ast>,
    pub implements_protocol: ImplementsProtocol<'ast>,
    pub protocol_members: &'ast [ProtocolMember<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProtocolMember<'ast> {
	FunctionDeclaration(FunctionDeclaration<'ast>),
    TypeAliasDeclaration(TypeAliasDeclaration<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub top_level: TopLevel<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation<'ast> {
    pub identifier: Identifier,
    pub literals: &'ast [Literal],	
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasDeclaration<'ast> {
	pub docs_comments: &'ast [DocsComments<'ast>],
    pub is_public: IsPublic,
    pub identifier: Identifier,
    pub generics: Generics<'ast>,
    pub type_literal: TypeLiteral<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression<'ast> {
    pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseIfClause<'ast> {
    pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseClause<'ast> {
	pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchExpression<'ast> {
    pub expression: Expression<'ast>,
    pub match_arms: &'ast [MatchArm<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm <'ast>{
    pub pattern: Pattern<'ast>,
    pub guard_if: Option<IfExpression<'ast>>,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopExpression<'ast> {
	pub loop_times: Option<Expression<'ast>>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpression <'ast>{
	pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement <'ast>{
	pub pattern: Pattern<'ast>,
    pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression<'ast> {
	pub expression: Expression<'ast>,
    pub pipeline_arms: &'ast [PipelineArm<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipelineArm <'ast>{
	pub pattern: Pattern<'ast>,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfLetExpression <'ast>{
	pub pattern: Pattern<'ast>,
    pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLetExpression<'ast> {
	pub pattern: Pattern<'ast>,
    pub expression: Expression<'ast>,
    pub block_expression: BlockExpression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipeExpression<'ast> {
	pub expression: Expression<'ast>,
    pub pipe_arms: &'ast [PipeArm<'ast>],
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipeArm<'ast> {
    pub pattern: Pattern<'ast>,
	pub guard_if: Expression<'ast>,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closer<'ast> {
	pub closer_params: Option<CloserParams<'ast>>,
    pub block_expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CloserParams<'ast>(pub &'ast [CloserParamItem<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub enum CloserParamItem<'ast> {
	ParamWithType(ParamWithType<'ast>),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Accesser<'ast>(pub &'ast [Identifier]);

#[derive(Debug, PartialEq, Clone)]
pub struct Params<'ast>(pub Option<ExpressionList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Expression<'ast>(pub LogicalOrExpr<'ast>);

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalOrExpr<'ast>(pub &'ast [LogicalAndExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalAndExpr<'ast>(pub &'ast [BitwiseOrExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseOrExpr<'ast>(pub &'ast [BitwiseXorExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseXorExpr<'ast>(pub &'ast [BitwiseAndExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseAndExpr<'ast>(pub &'ast [EqualityExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct EqualityExpr<'ast> {
	pub left: RelationalExpr<'ast>,
    pub operator: EqualityOperator,
    pub right: RelationalExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelationalExpr<'ast> {
    pub left: ShiftExpr<'ast>,
    pub operator: RelationalOperator,
    pub right: ShiftExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftExpr<'ast> {
	pub left: AdditiveExpr<'ast>,
    pub operator: ShiftOperator,
    pub right: AdditiveExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdditiveExpr<'ast> {
    pub left: MultiplicativeExpr<'ast>,
    pub oprator: AdditiveOperator,
    pub right: MultiplicativeExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MultiplicativeExpr<'ast> {
	pub left: PowerExpr<'ast>,
    pub operator: MultiplicativeOperator,
    pub right: PowerExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PowerExpr<'ast>(pub &'ast [PrefixExpr<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpr<'ast> {
	pub operators: &'ast [PrefixOperator],
    pub expression: PrimaryExpr<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpr<'ast> {
	BlockExpression(BlockExpression<'ast>),
    IfExpression(IfExpression<'ast>),
    MatchExpression(MatchExpression<'ast>),
    LoopExpression(LoopExpression<'ast>),
    WhileExpression(WhileExpression<'ast>),
    ForExpression(ForExpression<'ast>),
    PipeExpression(PipeExpression<'ast>),
    Accesser(Accesser<'ast>),
    Literal(Literal),
    FunctionCall(FunctionCall<'ast>),
    MethodCall(MethodCall<'ast>),
    FieldAccess(FieldAccess<'ast>),
    AwaitExpression(Expression<'ast>),
    TupleOrGroupedExpression(TupleOrGroupedExpression<'ast>),
    StructLiteral(StructLiteral<'ast>),
    Closer(Closer<'ast>),
    IfLetExpression(IfLetExpression<'ast>),
    WhileLetExpression(WhileLetExpression<'ast>),
    ArrayLiteral(ArrayLiteral<'ast>),
    IndexAccess(IndexAccess<'ast>),
    CastExpression(CastExpression<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall<'ast> {
	pub is_try: IsTry,
    pub accesser: Accesser<'ast>,
    pub params: Params<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall<'ast> {
	pub is_try: IsTry,
    pub accesser: Accesser<'ast>,
    pub identifier: Identifier,
    pub params: Params<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldAccess<'ast> {
	pub accesser: Accesser<'ast>,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleOrGroupedExpression<'ast>(pub Option<ExpressionList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructLiteral<'ast> {
	pub accesser: Accesser<'ast>,
    pub struct_literal_fields: Option<StructLiteralFields<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructLiteralFields<'ast>(pub &'ast [StructFieldInit<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct StructFieldInit<'ast> {
	pub field_name: Identifier,
    pub expression: Option<Expression<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLiteral<'ast>(pub Option<ExpressionList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct IndexAccess<'ast> {
	pub container: Expression<'ast>,
    pub index: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastExpression<'ast> {
	pub target: Expression<'ast>,
    pub type_literal: TypeLiteral<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'ast> {
	IfExpression(IfExpression<'ast>),
    MatchExpression(MatchExpression<'ast>),
    LoopExpression(LoopExpression<'ast>),
    WhileExpression(WhileExpression<'ast>),
    ForStatement(ForStatement<'ast>),
    ExpressionStatement(ExpressionStatement<'ast>),
    VariableDeclaration(VariableDeclaration<'ast>),
    ReturnStatement(Expression<'ast>),
    BreakStatement(Expression<'ast>),
    ContinueStatement,
    AssignmentStatement(AssignmentStatement<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement<'ast> {
	pub is_ignore: IsIgnore,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration<'ast> {
	pub variable_declaration_keyword: VariableDeclarationKeyword,
    pub pattern: Pattern<'ast>,
    pub type_literal: Option<TypeLiteral<'ast>>,
    pub variable_declaration_assignment: VariableDeclarationAssignment<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKeyword {
	Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationAssignment<'ast> {
	pub assignment_operator: AssignmentOperator,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStatement<'ast> {
	pub accesser: Accesser<'ast>,
    pub assignment_operator: AssignmentOperator,
    pub expression: Expression<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldDeclaration<'ast> {
    pub field_declaration_keyword: FieldDeclarationKeyword,
	pub identifier: Identifier,
    pub type_literal: TypeLiteral<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParamWithType<'ast> {
	Param {
        is_mut: IsMut,
        identifier: Identifier,
        type_literal: TypeLiteral<'ast>,
        default_value: Option<Expression<'ast>>,
    },
    This {
        is_mut: IsMut,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParamsWithTypes<'ast>(pub Option<ParamWithType<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct ParamWithTypesList<'ast>(pub &'ast [ParamsWithTypes<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub enum BlockExpression<'ast> {
    StatementList(&'ast [Statement<'ast>]),
    Expression(Expression<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsPublic {
	Public,
    Private,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
	StringLiteral(StringLiteral),
    CharLiteral(CharLiteral),
    NumLiteral(NumLiteral),
    BoolLiteral(BoolLiteral),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeLiteral<'ast> {
	Identifier {
        accesser: Accesser<'ast>,
        generics: Option<Generics<'ast>>,
    },
    ImplType(&'ast TypeLiteral<'ast>),
    TupleType(TupleType<'ast>),
    TypeOf(Expression<'ast>),
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

#[derive(Debug, PartialEq, Clone)]
pub struct GenericTypeArgs<'ast>(pub TypeLiteralList<'ast>);

#[derive(Debug, PartialEq, Clone)]
pub struct TupleType<'ast>(pub Option<TypeLiteralList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'ast> {
	Identifier(Identifier),
    Wild,
    TupleStructPattern(TupleStructPattern<'ast>),
    TuplePattern(TuplePattern<'ast>),
    StructPattern(StructPattern<'ast>),
    Accesser(Accesser<'ast>),
    Literal(Literal),
    RangePattern(RangePattern),
    BindingPattern(BindingPattern<'ast>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleStructPattern<'ast> {
	pub accesser: Accesser<'ast>,
    pub pattern_list: Option<PatternList<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TuplePattern<'ast>(pub Option<PatternList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructPattern<'ast> {
	pub accesser: Accesser<'ast>,
    pub struct_pattern_fields: Option<StructPatternFields<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructPatternFields<'ast>(pub &'ast [StructPatternField<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct StructPatternField<'ast> {
	pub identifier: Identifier,
    pub pattern: Option<Pattern<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum RangeOp {
	InRange,
    InRangeOr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BindingPattern<'ast> {
	pub identifier: Identifier,
    pub pattern: &'ast Pattern<'ast>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Generics<'ast>(pub GenericParamDefList<'ast>);

#[derive(Debug, PartialEq, Clone)]
pub struct GenericParamDefList<'ast>(pub &'ast [GenericParamDef<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct GenericParamDef<'ast> {
	pub identifier: Identifier,
    pub generic_bound: Option<GenericBound<'ast>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericBound<'ast>(pub &'ast [TypeLiteral<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementsProtocol<'ast>(pub Option<AccesserList<'ast>>);

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierList<'ast>(pub &'ast [Identifier]);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeLiteralList<'ast>(pub &'ast [TypeLiteral<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList<'ast>(pub &'ast [Expression<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct PatternList<'ast>(pub &'ast [Pattern<'ast>]);

#[derive(Debug, PartialEq, Clone)]
pub struct AccesserList<'ast>(pub &'ast [Accesser<'ast>]);
