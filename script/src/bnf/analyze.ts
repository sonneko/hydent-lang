import { readFileSync } from "fs";

import { BranchRule, Grammar, ProductRule, Rule } from "./ast";
import {
    astName,
    BranchParserFunction,
    fnName,
    IR,
    ParserFunction,
    ProductParserFunction,
    RustTokenTypeName,
    tokenName
} from "./ir";

// ----------------------------------------------------------------------------
// Utility Types & Helpers
// ----------------------------------------------------------------------------

type TokenSeq = RustTokenTypeName[];

function seqKey(seq: TokenSeq): string {
    return seq.join(",");
}

function parseSeqKey(key: string): TokenSeq {
    if (key === "") return [];
    return key.split(",") as RustTokenTypeName[];
}

function concatSeq(a: TokenSeq, b: TokenSeq): TokenSeq {
    const combined = [...a, ...b];
    return combined.slice(0, 2);
}

// ----------------------------------------------------------------------------
// TokenMap
// ----------------------------------------------------------------------------

class TokenMap {
    private map: Record<string, string>;

    public constructor(record?: Record<string, string>) {
        if (record !== undefined) {
            this.map = record;
            return;
        }
        const map = new Map<string, string>();
        try {
            const csv = readFileSync('../assets/token_map.csv', 'utf8');
            csv.split('\n').forEach(line => {
                const parts = line.split('"');
                if (parts.length >= 4 && parts[1] && parts[3]) {
                    map.set(parts[1], parts[3]);
                }
            });
        } catch (e) {
            console.error("Could not load token_map.csv, using empty map or mock.", e);
        }
        this.map = Object.fromEntries(map);
    }

    public get(key: string): RustTokenTypeName {
        const val = this.map[key];
        if (!val) {
            console.error(`Token definition for literal "${key}" not found.`);
            return tokenName("UnknownToken");
        }
        return tokenName(val);
    }
}

// ----------------------------------------------------------------------------
// Analyzer
// ----------------------------------------------------------------------------

export class Analyzer {
    private grammar: Grammar;
    private tokenMap: TokenMap;
    private ruleMap: Map<string, Rule>;

    public nullable: Set<string> = new Set();
    private firstSets: Map<string, Set<string>> = new Map();
    private followSets: Map<string, Set<string>> = new Map();

    public constructor(grammar: Grammar, tokenMap?: Record<string, string>) {
        this.grammar = grammar;
        this.tokenMap = new TokenMap(tokenMap);
        this.ruleMap = new Map();

        for (const rule of grammar) {
            this.ruleMap.set(rule.name, rule);
        }
    }

    public analyze(): IR {
        this.computeNullable();
        this.computeFirst();
        this.computeFollow();

        const funcs: ParserFunction[] = [];
        for (const rule of this.grammar) {
            switch (rule.kind) {
                case "Branch":
                    funcs.push(this.analyzeBranchRule(rule));
                    break;
                case "Product":
                    funcs.push(this.analyzeProductRule(rule));
                    break;
            }
        }
        return funcs;
    }

    public computeNullable() {
        let changed = true;
        while (changed) {
            changed = false;
            for (const rule of this.grammar) {
                const wasNullable = this.nullable.has(rule.name);
                let isNullable = false;

                if (rule.kind === "Branch") {
                    for (const v of rule.variants) {
                        if (this.nullable.has(v.name)) {
                            isNullable = true;
                            break;
                        }
                    }
                } else if (rule.kind === "Product") {
                    let allNullable = true;
                    if (rule.members.length === 0) {
                        allNullable = true;
                    } else {
                        for (const m of rule.members) {
                            if (m.kind === "Terminal") {
                                allNullable = false;
                                break;
                            } else {
                                if (m.type.modifier === "Option" || m.type.modifier === "List") {
                                    continue;
                                }
                                if (!this.nullable.has(m.type.name)) {
                                    allNullable = false;
                                    break;
                                }
                            }
                        }
                    }
                    isNullable = allNullable;
                }

                if (isNullable && !wasNullable) {
                    this.nullable.add(rule.name);
                    changed = true;
                }
            }
        }
    }

    private computeFirst() {
        for (const rule of this.grammar) {
            this.firstSets.set(rule.name, new Set());
        }

        let changed = true;
        while (changed) {
            changed = false;
            for (const rule of this.grammar) {
                const currentSet = this.firstSets.get(rule.name)!;
                const oldSize = currentSet.size;

                if (rule.kind === "Branch") {
                    for (const v of rule.variants) {
                        const variantFirst = this.firstSets.get(v.name);
                        if (variantFirst) {
                            for (const seqStr of variantFirst) {
                                currentSet.add(seqStr);
                            }
                        }
                    }
                } else if (rule.kind === "Product") {
                    let seqs: TokenSeq[] = [[]];

                    for (const m of rule.members) {
                        let memberFirsts: TokenSeq[] = [];

                        if (m.kind === "Terminal") {
                            const t = this.tokenMap.get(m.value);
                            memberFirsts = [[t]];
                        } else {
                            const refName = m.type.name;
                            const refModifier = m.type.modifier;

                            const subFirstStrs = this.firstSets.get(refName);
                            if (subFirstStrs) {
                                subFirstStrs.forEach(s => memberFirsts.push(parseSeqKey(s)));
                            }
                            
                            // MODIFIED: Added nullable rule check
                            if (refModifier === "List" || refModifier === "Option" || this.nullable.has(refName)) {
                                memberFirsts.push([]);
                            }
                        }

                        const nextSeqs: TokenSeq[] = [];
                        for (const base of seqs) {
                            for (const suffix of memberFirsts) {
                                if (base.length >= 2) {
                                    nextSeqs.push(base); 
                                } else {
                                    nextSeqs.push(concatSeq(base, suffix));
                                }
                            }
                        }
                        seqs = nextSeqs;
                        if (seqs.every(s => s.length >= 2)) break;
                    }

                    for (const s of seqs) {
                        if (s.length > 0) {
                             currentSet.add(seqKey(s));
                        }
                    }
                }

                if (currentSet.size !== oldSize) {
                    changed = true;
                }
            }
        }
    }

    private computeFollow() {
        for (const rule of this.grammar) {
            this.followSets.set(rule.name, new Set());
        }

        let changed = true;
        while (changed) {
            changed = false;
            for (const rule of this.grammar) {
                if (rule.kind !== "Product") continue;

                const members = rule.members;
                for (let i = 0; i < members.length; i++) {
                    const current = members[i];
                    if (current.kind === "Terminal") continue;

                    const targetName = current.type.name;
                    const targetFollow = this.followSets.get(targetName);
                    if (!targetFollow) continue;

                    const oldSize = targetFollow.size;
                    let allSubsequentNullable = true;
                    let lookaheadSeqs: TokenSeq[] = [[]];

                    for (let j = i + 1; j < members.length; j++) {
                        const next = members[j];
                        let nextFirsts: TokenSeq[] = [];

                        if (next.kind === "Terminal") {
                            nextFirsts = [[this.tokenMap.get(next.value)]];
                            allSubsequentNullable = false;
                        } else {
                            const nName = next.type.name;
                            const nMod = next.type.modifier;

                            const fs = this.firstSets.get(nName);
                            if (fs) fs.forEach(s => nextFirsts.push(parseSeqKey(s)));
                            
                            if (nMod !== "List" && nMod !== "Option" && !this.nullable.has(nName)) {
                                allSubsequentNullable = false;
                            } else {
                                nextFirsts.push([]);
                            }
                        }

                        const nextLookaheads: TokenSeq[] = [];
                        for (const base of lookaheadSeqs) {
                            for (const suffix of nextFirsts) {
                                nextLookaheads.push(concatSeq(base, suffix));
                            }
                        }
                        lookaheadSeqs = nextLookaheads;
                        if (!allSubsequentNullable) break;
                    }

                    for (const seq of lookaheadSeqs) {
                        if (seq.length > 0) {
                            targetFollow.add(seq[0] as string);
                        }
                    }

                    if (allSubsequentNullable) {
                        const ruleFollow = this.followSets.get(rule.name);
                        if (ruleFollow) {
                            for (const f of ruleFollow) {
                                targetFollow.add(f);
                            }
                        }
                    }

                    if (targetFollow.size !== oldSize) {
                        changed = true;
                    }
                }
            }
        }
    }

    private analyzeBranchRule(rule: BranchRule): BranchParserFunction {
        const mapPeek0: Record<string, string[]> = {};
        const mapPeek1: Record<string, Record<string, string[]>> = {};
        // NEW: Track sequences that end exactly at length 1
        const mapPeek1Fallback: Record<string, string[]> = {}; 

        for (const v of rule.variants) {
            const vSet = this.firstSets.get(v.name);
            const seqs: TokenSeq[] = [];
            if (vSet) {
                vSet.forEach(s => seqs.push(parseSeqKey(s)));
            }

            for (const seq of seqs) {
                if (seq.length === 0) continue;
                
                const t0 = seq[0] as string;
                if (!mapPeek0[t0]) mapPeek0[t0] = [];
                if (!mapPeek0[t0].includes(v.name)) mapPeek0[t0].push(v.name);

                if (seq.length > 1) {
                    const t1 = seq[1] as string;
                    if (!mapPeek1[t0]) mapPeek1[t0] = {};
                    if (!mapPeek1[t0][t1]) mapPeek1[t0][t1] = [];
                    if (!mapPeek1[t0][t1].includes(v.name)) mapPeek1[t0][t1].push(v.name);
                } else {
                    // NEW: Fallback logic for length-1 sequences
                    if (!mapPeek1Fallback[t0]) mapPeek1Fallback[t0] = [];
                    if (!mapPeek1Fallback[t0].includes(v.name)) mapPeek1Fallback[t0].push(v.name);
                }
            }
        }

        const branchesJudgebleInPeek0: BranchParserFunction["branchesJudgebleInPeek0"] = [];
        const branchesJudgebleInPeek1: BranchParserFunction["branchesJudgebleInPeek1"] = [];
        const branchesFallbackInPeek1: BranchParserFunction["branchesFallbackInPeek1"] = [];
        const branchesNeedBacktrack: BranchParserFunction["branchesNeedBacktrack"] = [];
        const expectedTerminals: RustTokenTypeName[] = [];

        for (const t0 in mapPeek0) {
            expectedTerminals.push(tokenName(t0));
            const variants = mapPeek0[t0];

            if (variants.length === 1) {
                branchesJudgebleInPeek0.push({
                    astTypeName: astName(variants[0]),
                    firstTerminal: tokenName(t0),
                });
            } else {
                // Peek0 conflict -> Check Peek1 and Fallbacks
                const secondMap = mapPeek1[t0] || {};
                const fallbacks = mapPeek1Fallback[t0] || [];

                // If multiple branches terminate at length 1 on the exact same token, that's ambiguous.
                if (fallbacks.length > 1) {
                    fallbacks.forEach(v => {
                        branchesNeedBacktrack.push({
                            astTypeName: astName(v),
                            firstTerminal: tokenName(t0),
                            secondTerminal: tokenName("_"),
                        });
                    });
                }

                for (const t1 in secondMap) {
                    const varsInT1 = secondMap[t1];
                    if (varsInT1.length === 1) {
                        branchesJudgebleInPeek1.push({
                            astTypeName: astName(varsInT1[0]),
                            firstTerminal: tokenName(t0),
                            secondTerminal: tokenName(t1),
                        });
                    } else {
                        varsInT1.forEach(v => {
                            branchesNeedBacktrack.push({
                                astTypeName: astName(v),
                                firstTerminal: tokenName(t0),
                                secondTerminal: tokenName(t1),
                            });
                        });
                    }
                }

                // NEW: If exactly 1 fallback exists, it's the unambiguous choice if Peek1 tokens miss
                if (fallbacks.length === 1) {
                    branchesFallbackInPeek1.push({
                        astTypeName: astName(fallbacks[0]),
                        firstTerminal: tokenName(t0),
                    });
                }
            }
        }

        const syncPointsTerminals: RustTokenTypeName[] = [];
        const follow = this.followSets.get(rule.name);
        if (follow) {
            follow.forEach(t => syncPointsTerminals.push(tokenName(t)));
        }

        return {
            kind: "branch",
            functionName: fnName(rule.name),
            astTypeName: astName(rule.name),
            expectedTerminals,
            syncPointsTerminals,
            branchesJudgebleInPeek0,
            branchesJudgebleInPeek1,
            branchesFallbackInPeek1, // Added to IR
            branchesNeedBacktrack
        };
    }

    private analyzeProductRule(rule: ProductRule): ProductParserFunction {
        const elements: ProductParserFunction["elements"] = [];
        for (const member of rule.members) {
            switch (member.kind) {
                case "Field":
                    switch (member.type.modifier) {
                        case "None":
                            elements.push({ kind: "normal", astTypeName: astName(member.type.name) });
                            break;
                        case "Option":
                            elements.push({ kind: "option", astTypeName: astName(member.type.name) });
                            break;
                        case "List":
                            elements.push({ kind: "repeat", astTypeName: astName(member.type.name) });
                            break;
                    }
                    break;
                case "Terminal":
                    elements.push({
                        kind: "terminal",
                        tokenTypeName: this.tokenMap.get(member.value),
                    });
                    break;
            }
        }

        const syncPointsTerminals: RustTokenTypeName[] = [];
        const follow = this.followSets.get(rule.name);
        if (follow) {
            follow.forEach(t => syncPointsTerminals.push(tokenName(t)));
        }

        return {
            kind: "product",
            functionName: fnName(rule.name),
            astTypeName: astName(rule.name),
            syncPointsTerminals,
            elements,
        };
    }
}

export function analyze(grammar: Grammar, tokenMap?: Record<string, string>): IR {
    const analyzer = new Analyzer(grammar, tokenMap);
    return analyzer.analyze();
}
