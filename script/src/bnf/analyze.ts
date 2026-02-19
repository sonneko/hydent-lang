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

// トークン列（長さ0, 1, 2）。First集合の要素として使用。
type TokenSeq = RustTokenTypeName[];

// 文字列化してSet/Mapのキーにするためのヘルパー
function seqKey(seq: TokenSeq): string {
    return seq.join(",");
}

function parseSeqKey(key: string): TokenSeq {
    if (key === "") return [];
    return key.split(",") as RustTokenTypeName[];
}

// 2つのトークン列を連結し、長さk=2に切り詰める
function concatSeq(a: TokenSeq, b: TokenSeq): TokenSeq {
    const combined = [...a, ...b];
    return combined.slice(0, 2);
}

// ----------------------------------------------------------------------------
// TokenMap
// ----------------------------------------------------------------------------

class TokenMap {
    private map: Record<string, string>;

    public constructor() {
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

class Analyzer {
    private grammar: Grammar;
    private tokenMap: TokenMap;
    private ruleMap: Map<string, Rule>;

    // 計算結果のキャッシュ
    private nullable: Set<string> = new Set();
    private firstSets: Map<string, Set<string>> = new Map(); // Key: ASTName, Value: Set<SeqString>
    private followSets: Map<string, Set<string>> = new Map(); // Key: ASTName, Value: Set<TokenString>

    public constructor(grammar: Grammar) {
        this.grammar = grammar;
        this.tokenMap = new TokenMap();
        this.ruleMap = new Map();

        for (const rule of grammar) {
            this.ruleMap.set(rule.name, rule);
        }
    }

    public analyze(): IR {
        // 1. 不動点反復法による各種セットの計算
        this.computeNullable();
        this.computeFirst();
        this.computeFollow();

        // 2. IR生成
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

    // --- Fixed Point Iteration: Nullable ---
    private computeNullable() {
        let changed = true;
        while (changed) {
            changed = false;
            for (const rule of this.grammar) {
                const wasNullable = this.nullable.has(rule.name);
                let isNullable = false;

                if (rule.kind === "Branch") {
                    // どれか一つのバリアントがNullableならNullable
                    for (const v of rule.variants) {
                        if (this.nullable.has(v.name)) {
                            isNullable = true;
                            break;
                        }
                    }
                } else if (rule.kind === "Product") {
                    // 全ての必須フィールドがNullableならNullable
                    let allNullable = true;
                    if (rule.members.length === 0) {
                        // 空プロダクトはNullable（文法上ありえる場合）
                        allNullable = true;
                    } else {
                        for (const m of rule.members) {
                            if (m.kind === "Terminal") {
                                allNullable = false;
                                break;
                            } else {
                                if (m.type.modifier === "Option" || m.type.modifier === "List") {
                                    continue; // これらは常に空を許容
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

    // --- Fixed Point Iteration: First(k=2) ---
    private computeFirst() {
        // 初期化
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
                    // プロダクトのFirst集合を計算 (要素の連結)
                    // Product: A B C ...
                    let seqs: TokenSeq[] = [[]]; // Start with epsilon sequence

                    for (const m of rule.members) {
                        let memberFirsts: TokenSeq[] = [];

                        if (m.kind === "Terminal") {
                            const t = this.tokenMap.get(m.value);
                            memberFirsts = [[t]];
                        } else {
                            // NonTerminal
                            const refName = m.type.name;
                            const refModifier = m.type.modifier;

                            // 参照先のFirstを取得
                            const subFirstStrs = this.firstSets.get(refName);
                            if (subFirstStrs) {
                                subFirstStrs.forEach(s => memberFirsts.push(parseSeqKey(s)));
                            }
                            
                            // Modifier処理
                            if (refModifier === "List" || refModifier === "Option") {
                                memberFirsts.push([]); // Epsilon
                            }
                        }

                        // 直積計算: current seqs * memberFirsts
                        const nextSeqs: TokenSeq[] = [];
                        for (const base of seqs) {
                            for (const suffix of memberFirsts) {
                                // baseが既に長さ2ならsuffixを見ても変わらないが、
                                // baseが短い場合は連結する
                                if (base.length >= 2) {
                                    nextSeqs.push(base); 
                                } else {
                                    nextSeqs.push(concatSeq(base, suffix));
                                }
                            }
                        }
                        seqs = nextSeqs;
                        
                        // 最適化: 全てのシーケンスが長さ2以上になったら以降のメンバーを見る必要はない
                        // (ただし、Nullableチェックがあるので単純にはbreakできないが、k=2の範囲では十分)
                        if (seqs.every(s => s.length >= 2)) break;
                    }

                    for (const s of seqs) {
                        if (s.length > 0) { // 空シーケンス(完全なepsilon)はFirst集合には含めないのが一般的だが、実装都合による
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

    // --- Fixed Point Iteration: Follow ---
    private computeFollow() {
        // 初期化
        for (const rule of this.grammar) {
            this.followSets.set(rule.name, new Set());
        }

        // ルート要素や特定の開始記号にEOFを入れるべきだが、
        // ここでは全ルールの依存関係から解決する。

        let changed = true;
        while (changed) {
            changed = false;
            for (const rule of this.grammar) {
                if (rule.kind !== "Product") continue;

                // Rule -> A B C
                // Follow(B) += First(C)
                // If Nullable(C), Follow(B) += Follow(C) ... and so on
                // If C is last, Follow(C) += Follow(Rule)

                const members = rule.members;
                for (let i = 0; i < members.length; i++) {
                    const current = members[i];
                    if (current.kind === "Terminal") continue;

                    const targetName = current.type.name;
                    const targetFollow = this.followSets.get(targetName);
                    if (!targetFollow) continue; // Should not happen

                    const oldSize = targetFollow.size;

                    // 後続の要素を確認
                    let allSubsequentNullable = true;
                    
                    // Look ahead at neighbors
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

                        // lookaheadSeqs に nextFirsts を結合
                        const nextLookaheads: TokenSeq[] = [];
                        for (const base of lookaheadSeqs) {
                            for (const suffix of nextFirsts) {
                                nextLookaheads.push(concatSeq(base, suffix));
                            }
                        }
                        lookaheadSeqs = nextLookaheads;
                        
                        // First(k=1)だけでFollowは十分なことが多いが、ここはFirst集合の計算結果(TokenSeq)を利用
                        // Follow集合自体は通常 Terminal の Set (k=1)
                        if (!allSubsequentNullable) break; // これ以降は見なくて良い
                    }

                    // Add First sets of subsequent parts to Follow
                    for (const seq of lookaheadSeqs) {
                        if (seq.length > 0) {
                            targetFollow.add(seq[0] as string); // Add first token
                        }
                    }

                    // If everything after is nullable, add Follow(Rule)
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

    // --- Generation Logic ---

    private analyzeBranchRule(rule: BranchRule): BranchParserFunction {
        // バリアントごとのFirst集合を取得し、衝突判定を行う
        // Map<FirstToken, Map<SecondToken, VariantName[]>>

        const mapPeek0: Record<string, string[]> = {};
        const mapPeek1: Record<string, Record<string, string[]>> = {};

        const variantInfo: { name: string, seqs: TokenSeq[] }[] = [];

        for (const v of rule.variants) {
            const vSet = this.firstSets.get(v.name);
            const seqs: TokenSeq[] = [];
            if (vSet) {
                vSet.forEach(s => seqs.push(parseSeqKey(s)));
            }
            variantInfo.push({ name: v.name, seqs });

            for (const seq of seqs) {
                if (seq.length === 0) continue; // Skip epsilon in branching logic (handled via nullable?)
                
                const t0 = seq[0] as string;
                if (!mapPeek0[t0]) mapPeek0[t0] = [];
                if (!mapPeek0[t0].includes(v.name)) mapPeek0[t0].push(v.name);

                if (seq.length > 1) {
                    const t1 = seq[1] as string;
                    if (!mapPeek1[t0]) mapPeek1[t0] = {};
                    if (!mapPeek1[t0][t1]) mapPeek1[t0][t1] = [];
                    if (!mapPeek1[t0][t1].includes(v.name)) mapPeek1[t0][t1].push(v.name);
                }
            }
        }

        const branchesJudgebleInPeek0: BranchParserFunction["branchesJudgebleInPeek0"] = [];
        const branchesJudgebleInPeek1: BranchParserFunction["branchesJudgebleInPeek1"] = [];
        const branchesNeedBacktrack: BranchParserFunction["branchesNeedBacktrack"] = [];
        const expectedTerminals: RustTokenTypeName[] = [];

        // 分類
        for (const t0 in mapPeek0) {
            expectedTerminals.push(tokenName(t0));
            const variants = mapPeek0[t0];

            if (variants.length === 1) {
                // Peek0だけで確定
                branchesJudgebleInPeek0.push({
                    astTypeName: astName(variants[0]),
                    firstTerminal: tokenName(t0),
                });
            } else {
                // Peek0で衝突 -> Peek1を確認
                const secondMap = mapPeek1[t0];
                if (!secondMap) {
                    // 情報不足、または全て長さ1で衝突 -> バックトラック
                    // 典型例: A -> "x", B -> "x"
                     variants.forEach(v => {
                        branchesNeedBacktrack.push({
                            astTypeName: astName(v),
                            firstTerminal: tokenName(t0),
                            secondTerminal: tokenName("Unknown"), // 便宜上
                        });
                    });
                    continue;
                }

                // t0 で始まる全てのバリアントについて、t1で区別できるか確認
                // 注意: variantがt0のみ(長さ1)の場合、t1は存在しない(Follow依存)。
                // ここでは単純化のため、First集合に含まれる情報だけで判断する。
                
                // secondMapに含まれる各t1について
                for (const t1 in secondMap) {
                    const varsInT1 = secondMap[t1];
                    if (varsInT1.length === 1) {
                        branchesJudgebleInPeek1.push({
                            astTypeName: astName(varsInT1[0]),
                            firstTerminal: tokenName(t0),
                            secondTerminal: tokenName(t1),
                        });
                    } else {
                        // Peek1でも衝突
                         varsInT1.forEach(v => {
                            // 重複追加を防ぐロジックが必要だが、IRの定義上リストなので追加
                            // ただし同じ組み合わせを除外するなどのフィルタが必要かも
                            branchesNeedBacktrack.push({
                                astTypeName: astName(v),
                                firstTerminal: tokenName(t0),
                                secondTerminal: tokenName(t1),
                            });
                        });
                    }
                }
            }
        }

        // Sync points (Follow set)
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
                            elements.push({
                                kind: "normal",
                                astTypeName: astName(member.type.name)
                            });
                            break;
                        case "Option":
                            elements.push({
                                kind: "option",
                                astTypeName: astName(member.type.name)
                            });
                            break;
                        case "List":
                            elements.push({
                                kind: "repeat",
                                astTypeName: astName(member.type.name)
                            });
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

export function analyze(grammar: Grammar): IR {
    const analyzer = new Analyzer(grammar);
    return analyzer.analyze();
}