pub mod errors;
pub mod parser;


#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a>(&'a str);

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral<'a>(&'a str);

#[derive(Debug, PartialEq, Clone)]
pub struct CharLiteral(char);

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
pub struct DocsComments<'a> {
    comments: Vec<&'a str>,
    annotation: Vec<Annotation<'a>>,
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
pub struct Grammer<'a> {
    top_level: TopLevel<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct TopLevel<'a> {
	children: Vec<TopLevelStatement<'a>>
}

#[derive(Debug, PartialEq, Clone)]
pub enum TopLevelStatement<'a> {
    ImportDeclaration(ImportDeclaration<'a>),
    StaticVariableDeclaration(StaticVariableDeclaration<'a>),
    ClassDeclaration(ClassDeclaration<'a>),
    EnumDeclaration(EnumDeclaration<'a>),
    StructDeclaration(StructDeclaration<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
    ProtocolDeclaration(ProtocolDeclaration<'a>),
    ModuleDeclaration(ModuleDeclaration<'a>),
    Annotation(Annotation<'a>),
    TypeAliasDeclaration(TypeAliasDeclaration<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportDeclaration<'a> {
	ImportSpecific(ImportSpecific<'a>, &'a str),
    ImportAllAs(ImportAllAs<'a>, &'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportSpecific<'a>(IdentifierList<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct ImportAllAs<'a>(Identifier<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct StaticVariableDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    generics: Generics<'a>,
    implements_protocol: ImplementsProtocol<'a>,
    function_declarations: Vec<FunctionDeclaration<'a>>,
    field_declarations: Vec<FieldDeclaration<'a>>,
    type_alias_declarations: Vec<TypeAliasDeclaration<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    generics: Generics<'a>,
    implements_protocol: ImplementsProtocol<'a>,
    enum_members: Vec<EnumMember<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumMember<'a> {
    EnumVariant(EnumVariant<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant<'a>(Option<TypeLiteralList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    struct_body: StructBody<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StructBody<'a> {
	StructBlockBody(StructBlockBody<'a>),
	StructTupleBody(StructTupleBody<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructBlockBody<'a> (Vec<FieldDeclaration<'a>>);


#[derive(Debug, PartialEq, Clone)]
pub struct StructTupleBody<'a> (TypeLiteralList<'a>);


#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_extern: IsExtern,
    is_public: IsPublic,
    is_async: IsAsync,
    identifier: Identifier<'a>,
    generics: Generics<'a>,
    params_with_types: ParamsWithTypes<'a>,
    return_type_literal: Option<TypeLiteral<'a>>,
    can_panics: CanPanics,
    block_expression: Option<BlockExpression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    generics: Generics<'a>,
    implements_protocol: ImplementsProtocol<'a>,
    protocol_members: Vec<ProtocolMember<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProtocolMember<'a> {
	FunctionDeclaration(FunctionDeclaration<'a>),
    TypeAliasDeclaration(TypeAliasDeclaration<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    top_level: TopLevel<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation<'a> {
    identifier: Identifier<'a>,
    literals: Vec<Literal<'a>>,	
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasDeclaration<'a> {
	docs_comments: Vec<DocsComments<'a>>,
    is_public: IsPublic,
    identifier: Identifier<'a>,
    generics: Generics<'a>,
    type_literal: TypeLiteral<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression<'a> {
    expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseIfClause<'a> {
    expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseClause<'a> {
	block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchExpression<'a> {
    expression: Expression<'a>,
    match_arms: Vec<MatchArm<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm<'a> {
    pattern: Pattern<'a>,
    guard_if: Option<IfExpression<'a>>,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoopExpression<'a> {
	loop_times: Option<Expression<'a>>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpression<'a> {
	expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement<'a> {
	pattern: Pattern<'a>,
    expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression<'a> {
	expression: Expression<'a>,
    pipeline_arms: Vec<PipelineArm<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipelineArm<'a> {
	pattern: Pattern<'a>,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfLetExpression<'a> {
	pattern: Pattern<'a>,
    expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileLetExpression<'a> {
	pattern: Pattern<'a>,
    expression: Expression<'a>,
    block_expression: BlockExpression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipeExpression<'a> {
	expression: Expression<'a>,
    pipe_arms: Vec<PipeArm<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PipeArm<'a> {
    pattern: Pattern<'a>,
	guard_if: Expression<'a>,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closer<'a> {
	closer_params: Option<CloserParams<'a>>,
    block_expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CloserParams<'a>(Vec<CloserParamItem<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub enum CloserParamItem<'a> {
	ParamWithType(ParamWithType<'a>),
    Identifier(Identifier<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Accesser<'a>(Vec<Identifier<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Params<'a>(Option<ExpressionList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Expression<'a>(LogicalOrExpr<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalOrExpr<'a>(Vec<LogicalAndExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalAndExpr<'a>(Vec<BitwiseOrExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseOrExpr<'a>(Vec<BitwiseXorExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseXorExpr<'a>(Vec<BitwiseAndExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct BitwiseAndExpr<'a>(Vec<EqualityExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct EqualityExpr<'a> {
	left: RelationalExpr<'a>,
    operator: EqualityOperator,
    right: RelationalExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelationalExpr<'a> {
    left: ShiftExpr<'a>,
    operator: RelationalOperator,
    right: ShiftExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShiftExpr<'a> {
	left: AdditiveExpr<'a>,
    operator: ShiftOperator,
    right: AdditiveExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdditiveExpr<'a> {
    left: MultiplicativeExpr<'a>,
    oprator: AdditiveOperator,
    right: MultiplicativeExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MultiplicativeExpr<'a> {
	left: PowerExpr<'a>,
    operator: MultiplicativeOperator,
    right: PowerExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PowerExpr<'a>(Vec<PrefixExpr<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpr<'a> {
	operators: Vec<PrefixOperator>,
    expression: PrimaryExpr<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpr<'a> {
	BlockExpression(BlockExpression<'a>),
    IfExpression(IfExpression<'a>),
    MatchExpression(MatchExpression<'a>),
    LoopExpression(LoopExpression<'a>),
    WhileExpression(WhileExpression<'a>),
    ForExpression(ForExpression<'a>),
    PipeExpression(PipeExpression<'a>),
    Accesser(Accesser<'a>),
    Literal(Literal<'a>),
    FunctionCall(FunctionCall<'a>),
    MethodCall(MethodCall<'a>),
    FieldAccess(FieldAccess<'a>),
    AwaitExpression(Expression<'a>),
    TupleOrGroupedExpression(TupleOrGroupedExpression<'a>),
    StructLiteral(StructLiteral<'a>),
    Closer(Closer<'a>),
    IfLetExpression(IfLetExpression<'a>),
    WhileLetExpression(WhileLetExpression<'a>),
    ArrayLiteral(ArrayLiteral<'a>),
    IndexAccess(IndexAccess<'a>),
    CastExpression(CastExpression<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall<'a> {
	is_try: IsTry,
    accesser: Accesser<'a>,
    params: Params<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall<'a> {
	is_try: IsTry,
    accesser: Accesser<'a>,
    identifier: Identifier<'a>,
    params: Params<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldAccess<'a> {
	accesser: Accesser<'a>,
    identifier: Identifier<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleOrGroupedExpression<'a>(Option<ExpressionList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructLiteral<'a> {
	accesser: Accesser<'a>,
    struct_literal_fields: Option<StructLiteralFields<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructLiteralFields<'a>(Vec<StructFieldInit<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructFieldInit<'a> {
	field_name: Identifier<'a>,
    expression: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLiteral<'a>(Option<ExpressionList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct IndexAccess<'a> {
	container: Expression<'a>,
    index: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CastExpression<'a> {
	target: Expression<'a>,
    type_literal: TypeLiteral<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
	IfExpression(IfExpression<'a>),
    MatchExpression(MatchExpression<'a>),
    LoopExpression(LoopExpression<'a>),
    WhileExpression(WhileExpression<'a>),
    ForStatement(ForStatement<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    ReturnStatement(Expression<'a>),
    BreakStatement(Expression<'a>),
    ContinueStatement,
    AssignmentStatement(AssignmentStatement<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement<'a> {
	is_ignore: IsIgnore,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration<'a> {
	variable_declaration_keyword: VariableDeclarationKeyword,
    pattern: Pattern<'a>,
    type_literal: Option<TypeLiteral<'a>>,
    variable_declaration_assignment: VariableDeclarationAssignment<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKeyword {
	Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarationAssignment<'a> {
	assignment_operator: AssignmentOperator,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStatement<'a> {
	accesser: Accesser<'a>,
    assignment_operator: AssignmentOperator,
    expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldDeclaration<'a> {
    field_declaration_keyword: FieldDeclarationKeyword,
	identifier: Identifier<'a>,
    type_literal: TypeLiteral<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParamWithType<'a> {
	Param {
        is_mut: IsMut,
        identifier: Identifier<'a>,
        type_literal: TypeLiteral<'a>,
        default_value: Option<Expression<'a>>,
    },
    This {
        is_mut: IsMut,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParamsWithTypes<'a>(Option<ParamWithType<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct ParamWithTypesList<'a>(Vec<ParamsWithTypes<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub enum BlockExpression<'a> {
    StatementList(Vec<Statement<'a>>),
    Expression(Expression<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IsPublic {
	Public,
    Private,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
	StringLiteral(StringLiteral<'a>),
    CharLiteral(CharLiteral),
    NumLiteral(NumLiteral),
    BoolLiteral(BoolLiteral),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeLiteral<'a> {
	Identifier {
        accesser: Accesser<'a>,
        generics: Option<Generics<'a>>,
    },
    ImplType(Box<TypeLiteral<'a>>),
    TupleType(TupleType<'a>),
    TypeOf(Expression<'a>),
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
pub struct GenericTypeArgs<'a>(TypeLiteralList<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct TupleType<'a>(Option<TypeLiteralList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'a> {
	Identifier(Identifier<'a>),
    Wild,
    TupleStructPattern(TupleStructPattern<'a>),
    TuplePattern(TuplePattern<'a>),
    StructPattern(StructPattern<'a>),
    Accesser(Accesser<'a>),
    Literal(Literal<'a>),
    RangePattern(RangePattern),
    BindingPattern(BindingPattern<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TupleStructPattern<'a> {
	accesser: Accesser<'a>,
    pattern_list: Option<PatternList<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TuplePattern<'a>(Option<PatternList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructPattern<'a> {
	accesser: Accesser<'a>,
    struct_pattern_fields: Option<StructPatternFields<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructPatternFields<'a>(Vec<StructPatternField<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct StructPatternField<'a> {
	identifier: Identifier<'a>,
    pattern: Option<Pattern<'a>>,
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
pub struct BindingPattern<'a> {
	identifier: Identifier<'a>,
    pattern: Box<Pattern<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Generics<'a>(GenericParamDefList<'a>);

#[derive(Debug, PartialEq, Clone)]
pub struct GenericParamDefList<'a>(Vec<GenericParamDef<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct GenericParamDef<'a> {
	identifier: Identifier<'a>,
    generic_bound: Option<GenericBound<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GenericBound<'a>(Vec<TypeLiteral<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct ImplementsProtocol<'a>(Option<AccesserList<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierList<'a>(Vec<Identifier<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct TypeLiteralList<'a>(Vec<TypeLiteral<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList<'a>(Vec<Expression<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct PatternList<'a>(Vec<Pattern<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct AccesserList<'a>(Vec<Accesser<'a>>);
