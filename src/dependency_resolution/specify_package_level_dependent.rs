// use crate::compiler::runtime::{Query, Engine};
// use crate::compiler::span::Span;
// use crate::parser::ast;

// pub struct SpecifyModuleLevelDependenciesQuery;
// impl Query for SpecifyModuleLevelDependenciesQuery {
//     type From = ast::Module;
//     type To = Vec<Span>;
//     fn run<E: Engine>(db: &E, mut ast: Self::From) -> Self::To {
//         let mut visitor = Visitor::new();
//         for statement in ast.TopLevelStatement().into_ref() {
//             if let Some(statement) = statement.expect_ImportDeclaration() {
//                 visitor.found.push(statement.StringLiteral().span);
//             } else if let Some(item_with_modifier) = statement.expect_ItemWithModifiers() {
//                 if let Some(module) = item_with_modifier.ItemDeclaration().expect_ModuleDeclaration() {
//                     visitor.visit_module(module);
//                 }
//             }
//         }
//         visitor.found
//     }
// }

// struct Visitor {
//     found: Vec<Span>,
// }

// impl Visitor {
//     fn new() -> Self {
//         Self { found: vec![] }
//     }

//     fn visit_module<'ast>(&mut self, module: &'ast mut ast::ModuleDeclaration) -> Option<()> {
//         let statements = module.TopLevelStatement();
//         for statement in statements.into_ref() {
//             if let Some(statement) = statement.expect_ImportDeclaration() {
//                 self.found.push(statement.StringLiteral().span);
//             } else if let Some(item_with_modifier) = statement.expect_ItemWithModifiers() {
//                 if let Some(module) = item_with_modifier.ItemDeclaration().expect_ModuleDeclaration() {
//                     self.visit_module(module);
//                 }
//             }
//         }
//         Some(())
//     }

// }
