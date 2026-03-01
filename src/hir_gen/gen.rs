// use crate::hir_gen::hir;
// use crate::parser::ast;

// pub struct HirGenerator {

// }

// impl HirGenerator {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn generate(mut module: ast::Module) -> hir::Module {
//         for statement in module.TopLevelStatement().into_ref() {
//             if let Some(import) = statement.expect_ImportDeclaration() {

//             } else if let Some(item_with_modifier) = statement.expect_ItemWithModifiers() {

//             } else if let Some(annotation) = statement.expect_Annotation() {

//             }
//         }

//         todo!()
//     }
// }
