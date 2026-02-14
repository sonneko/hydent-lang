import { Grammar, Rule } from './parse';

type Brand<K, T> = T & { readonly __brand: K };
export type RuleName = Brand<"RuleName", string>; // name on dsl
export type RustTypeName = Brand<"RustTypeName", string>; // PascalCase
export type RustFieldName = Brand<"RustFieldName", string>; // snake_case

export const Converter = {
    ruleName: (val: string): RuleName => val as RuleName,
    rustType: (val: string): RustTypeName => val as RustTypeName,
    rustField: (val: string): RustFieldName => val as RustFieldName,
};

export interface GrammarIR {
    rules: Map<RuleName, RuleIR>;
    hooks: Map<RustFieldName, HookIR>;
    terminals: Set<string>;
    firstSets: Map<RuleName, Set<string>>;
    followSets: Map<RuleName, Set<string>>;
}

export interface HookIR {
    methodName: RustFieldName;
    returnType: string;
    doc: string;
}

export type RuleIR = BranchRuleIR | ProductRuleIR;

export interface BranchRuleIR {
    kind: 'Branch';
    rustName: RustTypeName;
    variants: Array<{
        rustVariantName: RustTypeName;
        targetRule: RuleName;
    }>;
}

export interface ProductRuleIR {
    kind: 'Product';
    rustName: RustTypeName;
    elements: ElementIR[] | undefined;
}

export type ElementIR =
    | { kind: 'Terminal'; value: string }
    | {
        kind: 'NonTerminal';
        targetRule: RuleName;
        modifier: 'None' | 'List' | 'Option';
        hook: RustFieldName | null;
        rustFieldName: RustFieldName;
        isBoxed: boolean;
    };

// --- Analyzer Implementation ---

export class GrammarAnalyzer {
    private ir: GrammarIR;
    private rawRules: Map<string, Rule>;

    constructor() {
        this.ir = {
            rules: new Map(),
            hooks: new Map(),
            terminals: new Set(),
            firstSets: new Map(),
            followSets: new Map(),
        };
        this.rawRules = new Map();
    }

    public analyze(grammar: Grammar): GrammarIR {
        this.analyzeSyntax(grammar);
        return this.ir;
    }

    private analyzeSyntax(grammar: Grammar) {
        grammar.forEach(r => {
            const name = Converter.ruleName(r.name);
            this.rawRules.set(name, r);
        });

        for (const rule of grammar) {
            const ruleName = Converter.ruleName(rule.name);
            const rustName = GrammarAnalyzer.toPascalCase(rule.name);

            if (rule.kind === 'Branch') {
                const variants = rule.variants.map(v => ({
                    rustVariantName: GrammarAnalyzer.toPascalCase(v.name),
                    targetRule: Converter.ruleName(v.name),
                }));

                this.ir.rules.set(ruleName, {
                    kind: 'Branch',
                    rustName,
                    variants,
                });

            } else {
                const elements: ElementIR[] = [];

                if (rule.members === undefined) {
                    this.ir.rules.set(ruleName, {
                        kind: 'Product',
                        rustName,
                        elements: undefined,
                    });
                } else {
                    for (const member of rule.members) {
                        if (member.kind === 'Terminal') {
                            this.ir.terminals.add(member.value);
                            elements.push({ kind: 'Terminal', value: member.value });
                        } else {
                            const targetRuleName = Converter.ruleName(member.type.name);
                            const modifier = member.type.modifier;

                            const rustFieldName = GrammarAnalyzer.toRustField(member.name);

                            let hookName: RustFieldName | null = null;
                            if (member.note) {
                                hookName = GrammarAnalyzer.toRustField(member.note);

                                const typeName = GrammarAnalyzer.toPascalCase(member.type.name);
                                const returnType = modifier === 'List' ? `ArenaIter<${typeName}>`
                                    : modifier === 'Option' ? `Option<${typeName}>`
                                        : typeName;

                                this.ir.hooks.set(hookName, {
                                    methodName: hookName,
                                    returnType,
                                    doc: `Hook for field '${member.name}' in rule '${rule.name}'`
                                });
                            }

                            elements.push({
                                kind: 'NonTerminal',
                                targetRule: targetRuleName,
                                modifier,
                                hook: hookName,
                                rustFieldName,
                                isBoxed: false,
                            });
                        }
                    }

                    this.ir.rules.set(ruleName, {
                        kind: 'Product',
                        rustName,
                        elements,
                    });
                }
            }
        }
    }

    // --- Helpers ---

    private static toPascalCase(str: string): RustTypeName {
        const converted = str
            .replace(/(?:^|_)([a-z])/g, (_, char) => char.toUpperCase())
            .replace(/_/g, '');
        return Converter.rustType(converted);
    }

    private static toRustField(str: string): RustFieldName {
        const snake = str
            .replace(/([A-Z])/g, '_$1')
            .toLowerCase()
            .replace(/^_/, '');
        return Converter.rustField(snake);
    }
}

export function analyze(ast: Grammar): GrammarIR {
    const analyzer = new GrammarAnalyzer();
    return analyzer.analyze(ast);
}
