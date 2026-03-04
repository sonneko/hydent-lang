import { BranchParserFunction, ProductParserFunction, HookParserFunction, IR } from "./ir";

import { ParserGenerator } from "./gen_parser";
import { ASTTypeGenerator } from "./gen_ast_type";
import { ASTPrinterGenerator } from "./gen_ast_printer";

export function getUniqueVariants(func: BranchParserFunction) {
    const list = [
        ...func.branchesJudgebleInPeek0,
        ...func.branchesJudgebleInPeek1,
        ...(func.branchesFallbackInPeek1 || []),
        ...func.branchesNeedBacktrack
    ];
    const map = new Map();
    for (const item of list) {
        map.set(item.astTypeName, { name: item.astTypeName, isBoxed: item.isBoxed });
    }
    return Array.from(map.values()).sort((a, b) => a.name.localeCompare(b.name));
}


export function generate(ir: IR): [string, string, string] {
    const genParser = new ParserGenerator();
    const genAstType = new ASTTypeGenerator();
    const genPrinter = new ASTPrinterGenerator();
    return [
        genParser.generateParser(ir),
        genAstType.generateASTType(ir),
        genPrinter.generateAstPrinterImpl(ir),
    ]
}
