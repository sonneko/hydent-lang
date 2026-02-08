import { Grammar, Rule } from './parse';

export interface GrammarIR {
    rules: Map<string, RuleIR>;
    hooks: Map<string, HookIR>;

    analysis: {
        firstSets: Map<string, Set<string>>;
        followSets: Map<string, Set<string>>;
        isNullable: Map<string, boolean>;
        recursiveRules: Set<string>;
        syncPoints: Set<string>;
    };

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
    elements: ElementIR[];
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
            analysis: {
                firstSets: new Map(),
                followSets: new Map(),
                isNullable: new Map(),
                recursiveRules: new Set(),
                syncPoints: new Set(),
            },
            terminals: new Set(),
        };
        this.rawRules = new Map();
    }

    public analyze(grammar: Grammar): GrammarIR {
        grammar.forEach(r => this.rawRules.set(r.name, r));

        this.buildBasicIR(grammar);
        this.computeNullables();
        this.computeFirstSets();
        this.computeFollowSets();
        this.detectRecursionAndBox();

        return this.ir;
    }

    // --- Pass 1: Basic IR Construction ---

    private buildBasicIR(grammar: Grammar): void {
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

    // --- Pass 2: Nullable Calculation ---

    private computeNullables(): void {
        let changed = true;
        while (changed) {
            changed = false;
            for (const [name, rule] of this.ir.rules) {
                const currentNullable = this.ir.analysis.isNullable.get(name) || false;
                let newNullable = false;

                if (rule.kind === 'Branch') {
                    newNullable = rule.variants.some(v => this.ir.analysis.isNullable.get(v.targetRule));
                } else {
                    newNullable = rule.elements.every(elem => {
                        if (elem.kind === 'Terminal') return false;
                        if (elem.modifier === 'Option' || elem.modifier === 'List') return true;
                        return this.ir.analysis.isNullable.get(elem.targetRule) || false;
                    });
                }

                if (newNullable !== currentNullable) {
                    this.ir.analysis.isNullable.set(name, newNullable);
                    changed = true;
                }
            }
        }
    }

    // --- Pass 3: First Sets Calculation ---

    private computeFirstSets(): void {
        // Initialize
        for (const name of this.ir.rules.keys()) {
            this.ir.analysis.firstSets.set(name, new Set());
        }

        let changed = true;
        while (changed) {
            changed = false;
            for (const [name, rule] of this.ir.rules) {
                const firstSet = this.ir.analysis.firstSets.get(name)!;
                const originalSize = firstSet.size;

                if (rule.kind === 'Branch') {
                    for (const v of rule.variants) {
                        const targetFirst = this.ir.analysis.firstSets.get(v.targetRule);
                        if (targetFirst) {
                            targetFirst.forEach(t => firstSet.add(t));
                        }
                    }
                } else {
                    let allNullableSoFar = true;
                    for (const elem of rule.elements) {
                        if (!allNullableSoFar) break;

                        if (elem.kind === 'Terminal') {
                            firstSet.add(elem.value);
                            allNullableSoFar = false;
                        } else {
                            // NonTerminal
                            const targetFirst = this.ir.analysis.firstSets.get(elem.targetRule);
                            if (targetFirst) {
                                targetFirst.forEach(t => firstSet.add(t));
                            }

                            const isElemNullable =
                                elem.modifier === 'Option' ||
                                elem.modifier === 'List' ||
                                this.ir.analysis.isNullable.get(elem.targetRule);

                            if (!isElemNullable) {
                                allNullableSoFar = false;
                            }
                        }
                    }
                }

                if (firstSet.size !== originalSize) {
                    changed = true;
                }
            }
        }
    }

    // --- Pass 4: Follow Sets & Sync Points ---

    private computeFollowSets(): void {
        // Initialize
        for (const name of this.ir.rules.keys()) {
            this.ir.analysis.followSets.set(name, new Set());
        }


        let changed = true;
        while (changed) {
            changed = false;

            for (const [parentName, rule] of this.ir.rules) {
                if (rule.kind !== 'Product') continue;

                const parentFollow = this.ir.analysis.followSets.get(parentName)!;

                // A -> ... B C ...
                // First(C) is in Follow(B)
                // If C is nullable, Follow(A) is in Follow(B)

                let trailer = new Set(parentFollow);

                for (let i = rule.elements.length - 1; i >= 0; i--) {
                    const elem = rule.elements[i];

                    if (elem.kind === 'NonTerminal') {
                        const targetFollow = this.ir.analysis.followSets.get(elem.targetRule)!;
                        const originalSize = targetFollow.size;

                        trailer.forEach(t => targetFollow.add(t));

                        if (targetFollow.size !== originalSize) {
                            changed = true;
                        }

                        const isElemNullable =
                            elem.modifier === 'Option' ||
                            elem.modifier === 'List' ||
                            this.ir.analysis.isNullable.get(elem.targetRule);

                        if (!isElemNullable) {
                            trailer = new Set();
                        }

                        const targetFirst = this.ir.analysis.firstSets.get(elem.targetRule);
                        if (targetFirst) {
                            targetFirst.forEach(t => trailer.add(t));
                        }

                    } else {
                        trailer = new Set([elem.value]);
                        if (i === rule.elements.length - 1) {
                            this.ir.analysis.syncPoints.add(elem.value);
                        }
                    }
                }
            }
        }
    }

    // --- Pass 5: Recursion & Box Detection ---

    private detectRecursionAndBox(): void {
        for (const [ruleName, rule] of this.ir.rules) {
            if (rule.kind !== 'Product') continue;

            for (const elem of rule.elements) {
                if (elem.kind === 'Terminal') continue;
                if (elem.modifier === 'List') continue;

                if (this.checkReaches(elem.targetRule, ruleName, new Set())) {
                    elem.isBoxed = true;
                    this.ir.analysis.recursiveRules.add(ruleName);
                }
            }
        }
    }

    private checkReaches(current: string, target: string, visited: Set<string>): boolean {
        if (current === target) return true;
        if (visited.has(current)) return false;
        visited.add(current);

        const rule = this.ir.rules.get(current);
        if (!rule) return false;

        if (rule.kind === 'Branch') {
            return rule.variants.some(v => this.checkReaches(v.targetRule, target, visited));
        } else {
            // Product
            return rule.elements.some(elem => {
                if (elem.kind === 'Terminal') return false;
                if (elem.modifier === 'List') return false;
                return this.checkReaches(elem.targetRule, target, visited);
            });
        }
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
