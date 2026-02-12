import { Grammar, Rule } from './parse';

export interface GrammarIR {
    rules: Map<string, RuleIR>;
    hooks: Map<string, HookIR>;
    terminals: Set<string>;
}

export interface HookIR {
    methodName: string;
    returnType: string;
    doc: string;
}

export type RuleIR = BranchRuleIR | ProductRuleIR;

export interface BranchRuleIR {
    kind: 'Branch';
    rustName: string; // PascalCase
    variants: Array<{
        rustVariantName: string; // PascalCase
        targetRule: string;
    }>;
}

export interface ProductRuleIR {
    kind: 'Product';
    rustName: string; // PascalCase
    elements: ElementIR[] | undefined;
}

export type ElementIR =
    | { kind: 'Terminal'; value: string }
    | {
        kind: 'NonTerminal';
        targetRule: string;
        modifier: 'None' | 'List' | 'Option';
        hook: string | null;
        rustFieldName: string; // snake_case, sanitized
        isBoxed: boolean;
    };

// --- Constants & Utils ---

const RUST_KEYWORDS = new Set([
    'as', 'break', 'const', 'continue', 'crate', 'else', 'enum', 'extern', 'false', 'fn',
    'for', 'if', 'impl', 'in', 'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub',
    'ref', 'return', 'self', 'Self', 'static', 'struct', 'super', 'trait', 'true',
    'type', 'unsafe', 'use', 'where', 'while', 'async', 'await', 'dyn', 'abstract',
    'become', 'box', 'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized',
    'virtual', 'yield', 'try'
]);

// --- Analyzer Implementation ---

export class GrammarAnalyzer {
    private ir: GrammarIR;
    private rawRules: Map<string, Rule>;

    constructor() {
        this.ir = {
            rules: new Map(),
            hooks: new Map(),
            terminals: new Set(),
        };
        this.rawRules = new Map();
    }

    public analyze(grammar: Grammar): GrammarIR {
        grammar.forEach(r => this.rawRules.set(r.name, r));
        for (const rule of grammar) {
            const rustName = this.toPascalCase(rule.name);

            if (rule.kind === 'Branch') {
                const variants = rule.variants.map(v => ({
                    rustVariantName: this.toPascalCase(v.name),
                    targetRule: v.name,
                }));

                this.ir.rules.set(rule.name, {
                    kind: 'Branch',
                    rustName,
                    variants,
                });

            } else {
                // Product
                const elements: ElementIR[] = [];

                if (rule.members === undefined) {
                    this.ir.rules.set(rule.name, {
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
                            // Field
                            const targetRuleName = member.type.name;
                            const modifier = member.type.modifier;
                            const rustFieldName = this.sanitizeIdentifier(this.toSnakeCase(member.name));

                            // Hook
                            let hookName: string | null = null;
                            if (member.note) {
                                hookName = this.sanitizeIdentifier(this.toSnakeCase(member.note));

                                const returnTypeRaw = this.toPascalCase(targetRuleName);
                                const returnType = modifier === 'List' ? `ArenaIter<${returnTypeRaw}>`
                                    : modifier === 'Option' ? `Option<${returnTypeRaw}>`
                                        : returnTypeRaw;

                                this.ir.hooks.set(hookName, {
                                    methodName: hookName,
                                    returnType: returnType,
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

                    this.ir.rules.set(rule.name, {
                        kind: 'Product',
                        rustName,
                        elements,
                    });
                }
            }
        }

        return this.ir;
    }

    // --- Helpers ---

    private toPascalCase(str: string): string {
        // snake_case to PascalCase
        return str
            .replace(/(?:^|_)([a-z])/g, (_, char) => char.toUpperCase())
            .replace(/_/g, '');
    }

    private toSnakeCase(str: string): string {
        // PascalCase or camelCase to snake_case
        return str
            .replace(/([A-Z])/g, '_$1')
            .toLowerCase()
            .replace(/^_/, '');
    }

    private sanitizeIdentifier(name: string): string {
        if (RUST_KEYWORDS.has(name)) {
            return `r#${name}`;
        }
        return name;
    }
}

export function analyze(ast: Grammar): GrammarIR {
    const analyzer = new GrammarAnalyzer();
    return analyzer.analyze(ast);
}
