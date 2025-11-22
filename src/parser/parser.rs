use crate::parser::errors::ParseErr;
use crate::tokenizer::Token;
use crate::parser::{self as AST, Grammer};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
    errors: Vec<ParseErr<'a>>,
}

type Return<'a, T> = Result<T, ParseErr<'a>>;

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            position: 0,
            errors: Vec::new(),
        }
    }

    // --- エントリーポイント ---
    pub fn parse(&mut self) -> Result<Grammer<'a>, Vec<ParseErr>> {
        unimplemented!();
    }

    /// ```ebnf
    /// <grammer> ::= <top_level>
    /// ```
    fn parse_grammer(&mut self) -> Return<AST::Grammer> { unimplemented!(); }

    /// ```ebnf
    /// <top_level> ::= { <top_level_statement> }
    /// ```
    fn parse_top_level(&mut self) -> Return<AST::TopLevel> { unimplemented!(); }

    /// ```ebnf
    /// <top_level_statement> ::=
    ///        <import_declaration>
    ///      | <static_variable_declaration>
    ///      | <class_declaration>
    ///      | <enum_declaration>
    ///      | <struct_declaration>
    ///      | <function_declaration>
    ///      | <protocol_declaration>
    ///      | <module_declaration>
    ///      | <annotation>
    ///      | <type_alias_declaration>
    /// ```
    fn parse_top_level_statement(&mut self) -> Return<AST::TopLevelStatement> { unimplemented!(); }

    /// ```ebnf
    /// <import_declaration> ::= "import" ( <import_specific> | <import_all_as> ) "from" <STRING_LITERAL> ";"
    /// ```
    fn parse_import_declaration(&mut self) -> Return<AST::ImportDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <import_specific> ::= "{" <identifier_list> "}"
    /// ```
    fn parse_import_specific(&mut self) -> Return<AST::ImportSpecific> { unimplemented!(); }

    /// ```ebnf
    /// <import_all_as> ::= "*" "as" <IDENTIFIER>
    /// ```
    fn parse_import_all_as(&mut self) -> Return<AST::ImportAllAs> { unimplemented!(); }

    /// ```ebnf
    /// <static_variable_declaration> ::= <docs_comments> <is_public> "static" <IDENTIFIER> "=" <expression> ";"
    /// ```
    fn parse_static_variable_declaration(&mut self) -> Return<AST::StaticVariableDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <class_declaration> ::= <docs_comments> <is_public> "class" <IDENTIFIER> <generics> <implements_protocol> "{" { <function_declaration> | <field_declaration> | <type_alias_declaration> } "}"
    /// ```
    fn parse_class_declaration(&mut self) -> Return<AST::ClassDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <enum_declaration> ::= <docs_comments> <is_public> "enum" <IDENTIFIER> <generics> <implements_protocol> "{" { <enum_member> } "}"
    /// ```
    fn parse_enum_declaration(&mut self) -> Return<AST::EnumDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <enum_member> ::= <enum_variant> | <function_declaration>
    /// ```
    fn parse_enum_member(&mut self) -> Return<AST::EnumMember> { unimplemented!(); }

    /// ```ebnf
    /// <enum_variant> ::= <IDENTIFIER> ( "(" <type_literal_list> ")" )?
    /// ```
    fn parse_enum_variant(&mut self) -> Return<AST::EnumVariant> { unimplemented!(); }

    /// ```ebnf
    /// <struct_declaration> ::= <docs_comments> <is_public> "struct" <IDENTIFIER> <struct_body>
    /// ```
    fn parse_struct_declaration(&mut self) -> Return<AST::StructDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <struct_body> ::= <struct_block_body> | <struct_tuple_body> | ";"
    /// ```
    fn parse_struct_body(&mut self) -> Return<AST::StructBody> { unimplemented!(); }

    /// ```ebnf
    /// <struct_block_body> ::= "{" { <field_declaration> } "}"
    /// ```
    fn parse_struct_block_body(&mut self) -> Return<AST::StructBlockBody> { unimplemented!(); }

    /// ```ebnf
    /// <struct_tuple_body> ::= "(" <type_literal_list> ")"
    /// ```
    fn parse_struct_tuple_body(&mut self) -> Return<AST::StructTupleBody> { unimplemented!(); }

    /// ```ebnf
    /// <function_declaration> ::= <docs_comments> "extern"? <is_public> "async"? "fn" <IDENTIFIER> <generics> "(" <params_with_types>  ")" ( ":" <type_literal> )? "panics"? <block_expression>?
    /// ```
    fn parse_function_declaration(&mut self) -> Return<AST::FunctionDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <protocol_declaration> ::= <docs_comments> <is_public> "protocol" <IDENTIFIER> <implements_protocol> "{" { <protocol_member> } "}"
    /// ```
    fn parse_protocol_declaration(&mut self) -> Return<AST::ProtocolDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <protocol_member> ::= <function_declaration> | <type_alias_declaration>
    /// ```
    fn parse_protocol_member(&mut self) -> Return<AST::ProtocolMember> { unimplemented!(); }

    /// ```ebnf
    /// <module_declaration> ::= <docs_comments> <is_public> "module" <IDENTIFIER> "{" <top_level> "}"
    /// ```
    fn parse_module_declaration(&mut self) -> Return<AST::ModuleDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <annotation> ::= "@" <IDENTIFIER> { <literal> }
    /// ```
    fn parse_annotation(&mut self) -> Return<AST::Annotation> { unimplemented!(); }

    /// ```ebnf
    /// <type_alias_declaration> ::= <docs_comments> <is_public> "type" <IDENTIFIER> <generics>? "=" <type_literal> ";"
    /// ```
    fn parse_type_alias_declaration(&mut self) -> Return<AST::TypeAliasDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <if_expression> ::= "if" <expression> <block_expression> { <else_if_clause> } <else_clause>?
    /// ```
    fn parse_if_expression(&mut self) -> Return<AST::IfExpression> { unimplemented!(); }

    /// ```ebnf
    /// <else_if_clause> ::= "else" "if" <expression> <block_expression>
    /// ```
    fn parse_else_if_clause(&mut self) -> Return<AST::ElseIfClause> { unimplemented!(); }

    /// ```ebnf
    /// <else_clause> ::= "else" <block_expression>
    /// ```
    fn parse_else_clause(&mut self) -> Return<AST::ElseClause> { unimplemented!(); }

    /// ```ebnf
    /// <match_expression> ::= "match" <expression> "{" { <match_arm> } "}"
    /// ```
    fn parse_match_expression(&mut self) -> Return<AST::MatchExpression> { unimplemented!(); }

    /// ```ebnf
    /// <match_arm> ::= <pattern> ( "if" <expression> )? "=>" <expression>
    /// ```
    fn parse_match_arm(&mut self) -> Return<AST::MatchArm> { unimplemented!(); }

    /// ```ebnf
    /// <loop_expression> ::= "loop" <expression>? <block_expression>
    /// ```
    fn parse_loop_expression(&mut self) -> Return<AST::LoopExpression> { unimplemented!(); }

    /// ```ebnf
    /// <while_expression> ::= "while" <expression> <block_expression>
    /// ```
    fn parse_while_expression(&mut self) -> Return<AST::WhileExpression> { unimplemented!(); }

    /// ```ebnf
    /// <for_statement> ::= "for" <pattern> "in" <expression> <block_expression>
    /// ```
    fn parse_for_statement(&mut self) -> Return<AST::ForStatement> { unimplemented!(); }

    /// ```ebnf
    /// <for_expression> ::= "for" <expression> "{" { <pipeline_arm> } "}"
    /// ```
    fn parse_for_expression(&mut self) -> Return<AST::ForExpression> { unimplemented!(); }

    /// ```ebnf
    /// <pipeline_arm> ::= "|>" <pattern> "=>" <expression>
    /// ```
    fn parse_pipeline_arm(&mut self) -> Return<AST::PipelineArm> { unimplemented!(); }

    /// ```ebnf
    /// <if_let_expression> ::= "if" "let" <pattern> "=" <expression> <block_expression>
    /// ```
    fn parse_if_let_expression(&mut self) -> Return<AST::IfLetExpression> { unimplemented!(); }

    /// ```ebnf
    /// <while_let_expression> ::= "while" "let" <pattern> "=" <expression> <block_expression>
    /// ```
    fn parse_while_let_expression(&mut self) -> Return<AST::WhileLetExpression> { unimplemented!(); }

    /// ```ebnf
    /// <pipe_expression> ::= "pipe" <expression> "{" { <pipe_arm> } "}"
    /// ```
    fn parse_pipe_expression(&mut self) -> Return<AST::PipeExpression> { unimplemented!(); }

    /// ```ebnf
    /// <pipe_arm> ::= "|>" <pattern> ("if" <expression>)? "=>" <block_expression> | <expression>
    /// ```
    fn parse_pipe_arm(&mut self) -> Return<AST::PipeArm> { unimplemented!(); }

    /// ```ebnf
    /// <closer> ::= "(" <closer_params>? ")" "->" <block_expression>
    /// ```
    fn parse_closer(&mut self) -> Return<AST::Closer> { unimplemented!(); }

    /// ```ebnf
    /// <closer_params> ::= <closer_param_item> { "," <closer_param_item> }
    /// ```
    fn parse_closer_params(&mut self) -> Return<AST::CloserParams> { unimplemented!(); }

    /// ```ebnf
    /// <closer_param_item> ::= <param_with_type> | <IDENTIFIER>
    /// ```
    fn parse_closer_param_item(&mut self) -> Return<AST::CloserParamItem> { unimplemented!(); }

    /// ```ebnf
    /// <accesser> ::= <IDENTIFIER> { "::" <IDENTIFIER> }
    /// ```
    fn parse_accesser(&mut self) -> Return<AST::Accesser> { unimplemented!(); }

    /// ```ebnf
    /// <params> ::= <expression_list>?
    /// ```
    fn parse_params(&mut self) -> Return<AST::Params> { unimplemented!(); }

    /// ```ebnf
    /// <expression> ::= <logical_or_expr>
    /// ```
    fn parse_expression(&mut self) -> Return<AST::Expression> { unimplemented!(); }

    /// ```ebnf
    /// <logical_or_expr> ::= <logical_and_expr> { "||" <logical_and_expr> }
    /// ```
    fn parse_logical_or_expr(&mut self) -> Return<AST::LogicalOrExpr> { unimplemented!(); }

    /// ```ebnf
    /// <logical_and_expr> ::= <bitwise_or_expr> { "&&" <bitwise_or_expr> }
    /// ```
    fn parse_logical_and_expr(&mut self) -> Return<AST::LogicalAndExpr> { unimplemented!(); }

    /// ```ebnf
    /// <bitwise_or_expr> ::= <bitwise_xor_expr> { "|" <bitwise_xor_expr> }
    /// ```
    fn parse_bitwise_or_expr(&mut self) -> Return<AST::BitwiseOrExpr> { unimplemented!(); }

    /// ```ebnf
    /// <bitwise_xor_expr> ::= <bitwise_and_expr> { "^" <bitwise_and_expr> }
    /// ```
    fn parse_bitwise_xor_expr(&mut self) -> Return<AST::BitwiseXorExpr> { unimplemented!(); }

    /// ```ebnf
    /// <bitwise_and_expr> ::= <equality_expr> { "&" <equality_expr> }
    /// ```
    fn parse_bitwise_and_expr(&mut self) -> Return<AST::BitwiseAndExpr> { unimplemented!(); }

    /// ```ebnf
    /// <equality_expr> ::= <relational_expr> { ("==" | "!=") <relational_expr> }
    /// ```
    fn parse_equality_expr(&mut self) -> Return<AST::EqualityExpr> { unimplemented!(); }

    /// ```ebnf
    /// <relational_expr> ::= <shift_expr> { ("<" | "<=" | ">" | ">=") <shift_expr> }
    /// ```
    fn parse_relational_expr(&mut self) -> Return<AST::RelationalExpr> { unimplemented!(); }

    /// ```ebnf
    /// <shift_expr> ::= <additive_expr> { ("<<" | ">>") <additive_expr> }
    /// ```
    fn parse_shift_expr(&mut self) -> Return<AST::ShiftExpr> { unimplemented!(); }

    /// ```ebnf
    /// <additive_expr> ::= <multiplicative_expr> { ("+" | "-") <multiplicative_expr> }
    /// ```
    fn parse_additive_expr(&mut self) -> Return<AST::AdditiveExpr> { unimplemented!(); }

    /// ```ebnf
    /// <multiplicative_expr> ::= <power_expr> { ("*" | "/" | "%") <power_expr> }
    /// ```
    fn parse_multiplicative_expr(&mut self) -> Return<AST::MultiplicativeExpr> { unimplemented!(); }

    /// ```ebnf
    /// <power_expr> ::= <prefix_expr> { "**" <power_expr> }
    /// ```
    fn parse_power_expr(&mut self) -> Return<AST::PowerExpr> { unimplemented!(); }

    /// ```ebnf
    /// <prefix_expr> ::= (("!" | "~") <prefix_expr>) | <primary_expr>
    /// ```
    fn parse_prefix_expr(&mut self) -> Return<AST::PrefixExpr> { unimplemented!(); }

    /// ```ebnf
    /// <primary_expr> ::=
    ///      <block_expression>
    ///    | <if_expression>
    ///    | <match_expression>
    ///    | <loop_expression>
    ///    | <while_expression>
    ///    | <for_expression>
    ///    | <pipe_expression>
    ///    | <accesser>
    ///    | <literal>
    ///    | <function_call>
    ///    | <method_call>
    ///    | <field_access>
    ///    | "await" <expression>
    ///    | <tuple_or_grouped_expression>
    ///    | <struct_literal>
    ///    | <closer>
    ///    | <if_let_expression>
    ///    | <while_let_expression>
    ///    | <array_literal>
    ///    | <index_access>
    ///    | <cast_expression>
    /// ```
    fn parse_primary_expr(&mut self) -> Return<AST::PrimaryExpr> { unimplemented!(); }

    /// ```ebnf
    /// <function_call> ::= "try"? <accesser> "(" <params> ")"
    /// ```
    fn parse_function_call(&mut self) -> Return<AST::FunctionCall> { unimplemented!(); }

    /// ```ebnf
    /// <method_call> ::= "try"? <accesser> "." <IDENTIFIER> "(" <params> ")"
    /// ```
    fn parse_method_call(&mut self) -> Return<AST::MethodCall> { unimplemented!(); }

    /// ```ebnf
    /// <field_access> ::= <accesser> "." <IDENTIFIER>
    /// ```
    fn parse_field_access(&mut self) -> Return<AST::FieldAccess> { unimplemented!(); }

    /// ```ebnf
    /// <tuple_or_grouped_expression> ::= "(" <expression_list>? ")"
    /// ```
    fn parse_tuple_or_grouped_expression(&mut self) -> Return<AST::TupleOrGroupedExpression> { unimplemented!(); }

    /// ```ebnf
    /// <struct_literal> ::= <accesser> "{" <struct_literal_fields>? "}"
    /// ```
    fn parse_struct_literal(&mut self) -> Return<AST::StructLiteral> { unimplemented!(); }

    /// ```ebnf
    /// <struct_literal_fields> ::= <struct_field_init> { "," <struct_field_init> }
    /// ```
    fn parse_struct_literal_fields(&mut self) -> Return<AST::StructLiteralFields> { unimplemented!(); }

    /// ```ebnf
    /// <struct_field_init> ::= ( <IDENTIFIER> ":" <expression> ) | <IDENTIFIER>
    /// ```
    fn parse_struct_field_init(&mut self) -> Return<AST::StructFieldInit> { unimplemented!(); }

    /// ```ebnf
    /// <array_literal> ::= "[" <expression_list>? "]"
    /// ```
    fn parse_array_literal(&mut self) -> Return<AST::ArrayLiteral> { unimplemented!(); }

    /// ```ebnf
    /// <index_access> ::= <expression> "[" <expression> "]"
    /// ```
    fn parse_index_access(&mut self) -> Return<AST::IndexAccess> { unimplemented!(); }

    /// ```ebnf
    /// <cast_expression> ::= <expression> "as" <type_literal>
    /// ```
    fn parse_cast_expression(&mut self) -> Return<AST::CastExpression> { unimplemented!(); }

    /// ```ebnf
    /// <statement> ::=
    ///      <if_expression>
    ///    | <match_expression>
    ///    | <loop_expression>
    ///    | <while_expression>
    ///    | <for_statement>
    ///    | <expression_statement>
    ///    | <variable_declaration>
    ///    | "return" <expression> ";"
    ///    | "break" <expression> ";"
    ///    | "continue" ";"
    ///    | <assignment_statement>
    /// ```
    fn parse_statement(&mut self) -> Return<AST::Statement> { unimplemented!(); }

    /// ```ebnf
    /// <expression_statement> ::= "ignore"? <expression> ";"
    /// ```
    fn parse_expression_statement(&mut self) -> Return<AST::ExpressionStatement> { unimplemented!(); }

    /// ```ebnf
    /// <variable_declaration> ::= <variable_declaration_keyword> <pattern> ( ":" <type_literal> )? <variable_declaration_assignment>? ";"
    /// ```
    fn parse_variable_declaration(&mut self) -> Return<AST::VariableDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <variable_declaration_keyword> ::= "let" | "const"
    /// ```
    fn parse_variable_declaration_keyword(&mut self) -> Return<AST::VariableDeclarationKeyword> { unimplemented!(); }

    /// ```ebnf
    /// <variable_declaration_assignment> ::= <ASSIGNMENT_OPERATOR> <expression>
    /// ```
    fn parse_variable_declaration_assignment(&mut self) -> Return<AST::VariableDeclarationAssignment> { unimplemented!(); }

    /// ```ebnf
    /// <assignment_statement> ::= <accesser> <ASSIGNMENT_OPERATOR> <expression> ";"
    /// ```
    fn parse_assignment_statement(&mut self) -> Return<AST::AssignmentStatement> { unimplemented!(); }

    /// ```ebnf
    /// <field_declaration> ::= ( "final" | "mut" ) <IDENTIFIER> ":" <type_literal> ";"
    /// ```
    fn parse_field_declaration(&mut self) -> Return<AST::FieldDeclaration> { unimplemented!(); }

    /// ```ebnf
    /// <param_with_type> ::= "mut"? ( <IDENTIFIER> ":" <type_literal> ( "=" <expression> )? ) | "this"
    /// ```
    fn parse_param_with_type(&mut self) -> Return<AST::ParamWithType> { unimplemented!(); }

    /// ```ebnf
    /// <params_with_types> ::= <param_with_types_list>?
    /// ```
    fn parse_params_with_types(&mut self) -> Return<AST::ParamsWithTypes> { unimplemented!(); }

    /// ```ebnf
    /// <param_with_types_list> ::= <param_with_type> { "," <param_with_type> }
    /// ```
    fn parse_param_with_types_list(&mut self) -> Return<AST::ParamWithTypesList> { unimplemented!(); }

    /// ```ebnf
    /// <block_expression> ::= "{" { <statement> } <expression>? "}"
    /// ```
    fn parse_block_expression(&mut self) -> Return<AST::BlockExpression> { unimplemented!(); }

    /// ```ebnf
    /// <is_public> ::= "pub" | "";
    /// ```
    fn parse_is_public(&mut self) -> Return<AST::IsPublic> { unimplemented!(); }

    /// ```ebnf
    /// <literal> ::= <STRING_LITERAL> | <CHAR_LITERAL> | <NUM_LITERAL> | <BOOL_LITERAL>
    /// ```
    fn parse_literal(&mut self) -> Return<AST::Literal> { unimplemented!(); }

    /// ```ebnf
    /// <type_literal> ::=
    ///      <accesser> <generic_type_args>?
    ///    | "impl" <type_literal>
    ///    | "typeof" <expression>
    ///    | "Bool"
    ///    | "Int"
    ///    | "DoubleInt"
    ///    | "Float"
    ///    | "DoubleFloat"
    ///    | "Char"
    ///    | "Usize"
    ///    | "Any"
    ///    | <tuple_type>
    ///    | "Never"
    ///    | "Void"
    /// ```
    fn parse_type_literal(&mut self) -> Return<AST::TypeLiteral> { unimplemented!(); }

    /// ```ebnf
    /// <generic_type_args> ::= "<" <type_literal_list> ">"
    /// ```
    fn parse_generic_type_args(&mut self) -> Return<AST::GenericTypeArgs> { unimplemented!(); }

    /// ```ebnf
    /// <tuple_type> ::= "(" <type_literal_list>? ")"
    /// ```
    fn parse_tuple_type(&mut self) -> Return<AST::TupleType> { unimplemented!(); }

    /// ```ebnf
    /// <docs_comments> ::= { <DOCS_COMMENT> } { <annotation> }
    /// ```
    fn parse_docs_comments(&mut self) -> Return<AST::DocsComments> { unimplemented!(); }

    /// ```ebnf
    /// <pattern> ::=
    ///      <IDENTIFIER>
    ///    | "_"
    ///    | <tuple_struct_pattern>
    ///    | <tuple_pattern>
    ///    | <struct_pattern>
    ///    | <accesser>
    ///    | <literal>
    ///    | <range_pattern>
    ///    | <binding_pattern>
    /// ```
    fn parse_pattern(&mut self) -> Return<AST::Pattern> { unimplemented!(); }

    /// ```ebnf
    /// <tuple_struct_pattern> ::= <accesser> "(" <pattern_list>? ")"
    /// ```
    fn parse_tuple_struct_pattern(&mut self) -> Return<AST::TupleStructPattern> { unimplemented!(); }

    /// ```ebnf
    /// <tuple_pattern> ::= "(" <pattern_list>? ")"
    /// ```
    fn parse_tuple_pattern(&mut self) -> Return<AST::TuplePattern> { unimplemented!(); }

    /// ```ebnf
    /// <struct_pattern> ::= <accesser> "{" <struct_pattern_fields>? "}"
    /// ```
    fn parse_struct_pattern(&mut self) -> Return<AST::StructPattern> { unimplemented!(); }

    /// ```ebnf
    /// <struct_pattern_fields> ::= <struct_pattern_field> { "," <struct_pattern_field> }
    /// ```
    fn parse_struct_pattern_fields(&mut self) -> Return<AST::StructPatternFields> { unimplemented!(); }

    /// ```ebnf
    /// <struct_pattern_field> ::= ( <IDENTIFIER> ":" <pattern> ) | <IDENTIFIER>
    /// ```
    fn parse_struct_pattern_field(&mut self) -> Return<AST::StructPatternField> { unimplemented!(); }

    /// ```ebnf
    /// <range_pattern> ::= ( <CHAR_LITERAL> | <NUM_LITERAL> ) <range_op> ( <CHAR_LITERAL> | <NUM_LITERAL> )
    /// ```
    fn parse_range_pattern(&mut self) -> Return<AST::RangePattern> { unimplemented!(); }

    /// ```ebnf
    /// <range_op> ::= ".." | "..="
    /// ```
    fn parse_range_op(&mut self) -> Return<AST::RangeOp> { unimplemented!(); }

    /// ```ebnf
    /// <binding_pattern> ::= <IDENTIFIER> "@" <pattern>
    /// ```
    fn parse_binding_pattern(&mut self) -> Return<AST::BindingPattern> { unimplemented!(); }

    /// ```ebnf
    /// <generics> ::= "<" <generic_param_def_list> ">"
    /// ```
    fn parse_generics(&mut self) -> Return<AST::Generics> { unimplemented!(); }

    /// ```ebnf
    /// <generic_param_def_list> ::= <generic_param_def> { "," <generic_param_def> }
    /// ```
    fn parse_generic_param_def_list(&mut self) -> Return<AST::GenericParamDefList> { unimplemented!(); }

    /// ```ebnf
    /// <generic_param_def> ::= <IDENTIFIER> ( ":" <generic_bound> )?
    /// ```
    fn parse_generic_param_def(&mut self) -> Return<AST::GenericParamDef> { unimplemented!(); }

    /// ```ebnf
    /// <generic_bound> ::= <type_literal> { "&" <type_literal> }
    /// ```
    fn parse_generic_bound(&mut self) -> Return<AST::GenericBound> { unimplemented!(); }

    /// ```ebnf
    /// <implements_protocol> ::= ( ":" <accesser_list> )?
    /// ```
    fn parse_implements_protocol(&mut self) -> Return<AST::ImplementsProtocol> { unimplemented!(); }

    /// ```ebnf
    /// <identifier_list> ::= <IDENTIFIER> { "," <IDENTIFIER> }
    /// ```
    fn parse_identifier_list(&mut self) -> Return<AST::IdentifierList> { unimplemented!(); }

    /// ```ebnf
    /// <type_literal_list> ::= <type_literal> { "," <type_literal> }
    /// ```
    fn parse_type_literal_list(&mut self) -> Return<AST::TypeLiteralList> { unimplemented!(); }

    /// ```ebnf
    /// <expression_list> ::= <expression> { "," <expression> }
    /// ```
    fn parse_expression_list(&mut self) -> Return<AST::ExpressionList> { unimplemented!(); }

    /// ```ebnf
    /// <pattern_list> ::= <pattern> { "," <pattern> }
    /// ```
    fn parse_pattern_list(&mut self) -> Return<AST::PatternList> { unimplemented!(); }

    /// ```ebnf
    /// <accesser_list> ::= <accesser> { "," <accesser> }
    /// ```
    fn parse_accesser_list(&mut self) -> Return<AST::AccesserList> { unimplemented!(); }
}
