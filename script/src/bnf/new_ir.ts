export type Brand<K, T> = T & { readonly __brand: K };

export const fnName = (s: string) => s as RustFunctionName;
export const astName = (s: string) => s as RustASTTypeName;
export const tokenName = (s: string) => s as RustTokenTypeName;

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
