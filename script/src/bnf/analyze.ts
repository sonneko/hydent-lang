import { Grammar, Rule, ProductMember } from './parse';
import { readFileSync } from 'node:fs';

// --- Branded Types ---
type Brand<K, T> = T & { readonly __brand: K };
export type RuleName = Brand<"RuleName", string>;
export type RustTypeName = Brand<"RustTypeName", string>;
export type RustFieldName = Brand<"RustFieldName", string>;
export type TokenPattern = Brand<"TokenPattern", string>;

export const Converter = {
    ruleName: (val: string): RuleName => val as RuleName,
    rustType: (val: string): RustTypeName => val as RustTypeName,
    rustField: (val: string): RustFieldName => val as RustFieldName,
    tokenPattern: (val: string): TokenPattern => val as TokenPattern,
};

// --- IR Types ---
export type ParserAction =
    | { kind: 'Call', target: RuleName, variantName: RustTypeName }
    | { kind: 'Backtrack', candidates: Array<{ target: RuleName, variantName: RustTypeName }> }
    | { kind: 'Error' };

export interface DecisionNode {
    peekIndex: 0 | 1;
    cases: Map<TokenPattern, DecisionNode | ParserAction>;
    default: ParserAction;
}

export interface GrammarIR {
    rules: Map<RuleName, RuleIR>;
    hooks: Map<RustFieldName, HookIR>;
    terminals: Set<string>;
    firstSets: Map<RuleName, Set<TokenPattern>>;
    // rule -> p0 -> Set<p1>
    first2Sets: Map<RuleName, Map<TokenPattern, Set<TokenPattern>>>;
}

export type RuleIR = BranchRuleIR | ProductRuleIR;

export interface BranchRuleIR {
    kind: 'Branch';
    rustName: RustTypeName;
    strategy: 'LL1' | 'LL2' | 'Ambiguous';
    decisionTree: DecisionNode;
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

export interface HookIR {
    methodName: RustFieldName;
    returnType: string;
    doc: string;
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

export class GrammarAnalyzer {
    private ir: GrammarIR;
    private rawRules: Map<RuleName, Rule>;
    private tokenMap: Map<string, string>;

    constructor() {
        this.ir = {
            rules: new Map(),
            hooks: new Map(),
            terminals: new Set(),
            firstSets: new Map(),
            first2Sets: new Map(),
        };
        this.rawRules = new Map();
        this.tokenMap = this.loadTokenMap();
    }

    public analyze(grammar: Grammar): GrammarIR {
        grammar.forEach(r => this.rawRules.set(Converter.ruleName(r.name), r));

        // 1. LL(1) 集合の計算
        this.computeFirstSets();
        // 2. LL(2) 集合の計算
        this.computeFirst2Sets();

        // 3. 各ルールの解析
        for (const rule of grammar) {
            const ruleName = Converter.ruleName(rule.name);
            if (rule.kind === 'Branch') {
                this.ir.rules.set(ruleName, this.analyzeBranch(rule));
            } else {
                this.ir.rules.set(ruleName, this.analyzeProduct(rule));
            }
        }

        return this.ir;
    }

    private analyzeBranch(rule: Extract<Rule, { kind: 'Branch' }>): BranchRuleIR {
        const ruleName = Converter.ruleName(rule.name);
        const rustName = this.toPascalCase(rule.name);
        const variants = rule.variants.map(v => ({
            rustVariantName: this.toPascalCase(v.name),
            targetRule: Converter.ruleName(v.name),
        }));

        const p0Cases = new Map<TokenPattern, DecisionNode | ParserAction>();
        let overallStrategy: 'LL1' | 'LL2' | 'Ambiguous' = 'LL1';

        // トークンごとにどのVariantが該当するかを集計
        const tokenToVariants = new Map<TokenPattern, typeof variants>();
        for (const v of variants) {
            const firsts = this.ir.firstSets.get(v.targetRule) || new Set();
            for (const f of firsts) {
                const list = tokenToVariants.get(f) || [];
                list.push(v);
                tokenToVariants.set(f, list);
            }
        }

        for (const [p0, candidates] of tokenToVariants) {
            if (candidates.length === 1) {
                // LL(1) で解決
                p0Cases.set(p0, { kind: 'Call', target: candidates[0].targetRule, variantName: candidates[0].rustVariantName });
            } else {
                // 衝突発生 -> LL(2) を試行
                if (overallStrategy === 'LL1') overallStrategy = 'LL2';

                const p1Cases = new Map<TokenPattern, DecisionNode | ParserAction>();
                const p1ToVariant = new Map<TokenPattern, typeof variants>();
                let canResolveWithLL2 = true;

                for (const v of candidates) {
                    const seconds = this.ir.first2Sets.get(v.targetRule)?.get(p0) || new Set([Converter.tokenPattern("_")]);
                    for (const s of seconds) {
                        const list = p1ToVariant.get(s) || [];
                        list.push(v);
                        p1ToVariant.set(s, list);
                    }
                }

                for (const [p1, vList] of p1ToVariant) {
                    if (vList.length === 1) {
                        p1Cases.set(p1, { kind: 'Call', target: vList[0].targetRule, variantName: vList[0].rustVariantName });
                    } else {
                        // LL(2) でも衝突
                        canResolveWithLL2 = false;
                        p1Cases.set(p1, { kind: 'Backtrack', candidates: vList.map(v => ({ target: v.targetRule, variantName: v.rustVariantName })) });
                    }
                }

                if (!canResolveWithLL2) overallStrategy = 'Ambiguous';

                p0Cases.set(p0, {
                    peekIndex: 1,
                    cases: p1Cases,
                    default: { kind: 'Backtrack', candidates: candidates.map(v => ({ target: v.targetRule, variantName: v.rustVariantName })) }
                } as DecisionNode);
            }
        }

        return {
            kind: 'Branch',
            rustName,
            strategy: overallStrategy,
            variants,
            decisionTree: {
                peekIndex: 0,
                cases: p0Cases,
                default: { kind: 'Backtrack', candidates: variants.map(v => ({ target: v.targetRule, variantName: v.rustVariantName })) }
            }
        };
    }

    private computeFirstSets() {
        for (const name of this.rawRules.keys()) this.ir.firstSets.set(name, new Set());
        let changed = true;
        while (changed) {
            changed = false;
            for (const [name, rule] of this.rawRules) {
                const set = this.ir.firstSets.get(name)!;
                const size = set.size;
                if (rule.kind === 'Branch') {
                    rule.variants.forEach(v => this.ir.firstSets.get(Converter.ruleName(v.name))?.forEach(f => set.add(f)));
                } else if (rule.members && rule.members.length > 0) {
                    const m = rule.members[0];
                    if (m.kind === 'Terminal') set.add(this.resolveTokenPattern(m.value));
                    else this.ir.firstSets.get(Converter.ruleName(m.type.name))?.forEach(f => set.add(f));
                }
                if (set.size > size) changed = true;
            }
        }
    }

    private computeFirst2Sets() {
        for (const name of this.rawRules.keys()) this.ir.first2Sets.set(name, new Map());
        let changed = true;
        while (changed) {
            changed = false;
            for (const [name, rule] of this.rawRules) {
                const ruleFirst2 = this.ir.first2Sets.get(name)!;
                const beforeTotal = this.countMapSets(ruleFirst2);

                if (rule.kind === 'Branch') {
                    for (const v of rule.variants) {
                        const targetFirst2 = this.ir.first2Sets.get(Converter.ruleName(v.name));
                        targetFirst2?.forEach((p1s, p0) => {
                            const s = ruleFirst2.get(p0) || new Set();
                            p1s.forEach(p1 => s.add(p1));
                            ruleFirst2.set(p0, s);
                        });
                    }
                } else if (rule.members && rule.members.length >= 1) {
                    const m1 = rule.members[0];
                    if (m1.kind === 'Terminal') {
                        const p0 = this.resolveTokenPattern(m1.value);
                        const s = ruleFirst2.get(p0) || new Set();
                        if (rule.members.length >= 2) {
                            const m2 = rule.members[1];
                            if (m2.kind === 'Terminal') s.add(this.resolveTokenPattern(m2.value));
                            else this.ir.firstSets.get(Converter.ruleName(m2.type.name))?.forEach(f => s.add(f));
                        } else {
                            s.add(Converter.tokenPattern("_")); // 2枚目がない
                        }
                        ruleFirst2.set(p0, s);
                    } else {
                        // 非終端文字で始まる場合、その非終端文字のFirst2を継承
                        const targetFirst2 = this.ir.first2Sets.get(Converter.ruleName(m1.type.name));
                        targetFirst2?.forEach((p1s, p0) => {
                            const s = ruleFirst2.get(p0) || new Set();
                            p1s.forEach(p1 => s.add(p1));
                            ruleFirst2.set(p0, s);
                        });
                    }
                }
                if (this.countMapSets(ruleFirst2) > beforeTotal) changed = true;
            }
        }
    }

    private countMapSets(m: Map<any, Set<any>>): number {
        let count = 0;
        m.forEach(s => count += s.size);
        return count;
    }

    // --- 以下、既存の analyzeProduct, loadTokenMap, resolveTokenPattern 等 ---
    private analyzeProduct(rule: Extract<Rule, { kind: 'Product' }>): ProductRuleIR {
        const elements: ElementIR[] = [];
        if (!rule.members) return { kind: 'Product', rustName: this.toPascalCase(rule.name), elements: undefined };

        for (const m of rule.members) {
            if (m.kind === 'Terminal') {
                elements.push({ kind: 'Terminal', value: m.value });
            } else {
                const targetName = Converter.ruleName(m.type.name);
                const rustFieldName = this.toRustField(m.name);
                let hook: RustFieldName | null = null;
                if (m.note) {
                    hook = this.toRustField(m.note);
                    this.ir.hooks.set(hook, {
                        methodName: hook,
                        doc: `Hook for ${m.name}`,
                        returnType: m.type.modifier === 'List' ? `ArenaIter<${this.toPascalCase(m.type.name)}>` : this.toPascalCase(m.type.name)
                    });
                }
                elements.push({
                    kind: 'NonTerminal',
                    targetRule: targetName,
                    modifier: m.type.modifier,
                    rustFieldName,
                    hook,
                    isBoxed: false
                });
            }
        }
        return { kind: 'Product', rustName: this.toPascalCase(rule.name), elements };
    }

    private loadTokenMap(): Map<string, string> {
        const map = new Map<string, string>();
        try {
            const csv = readFileSync('../assets/token_map.csv', 'utf8');
            csv.split('\n').forEach(line => {
                const parts = line.split('"');
                if (parts[1] && parts[3]) map.set(parts[1], parts[3]);
            });
        } catch (e) {}
        return map;
    }

    private resolveTokenPattern(raw: string): TokenPattern {
        const mapped = this.tokenMap.get(raw);
        return Converter.tokenPattern(mapped ? `Some(${mapped})` : `Some(Token::Unknown("${raw}"))`);
    }

    private toPascalCase(str: string): RustTypeName {
        return Converter.rustType(str.replace(/(?:^|_)([a-z])/g, (_, g) => g.toUpperCase()));
    }

    private toRustField(str: string): RustFieldName {
        return Converter.rustField(str.replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^_/, ""));
    }
}

export function analyze(grammar: Grammar): GrammarIR {
    const analyzer = new GrammarAnalyzer();
    return analyzer.analyze(grammar);
}
