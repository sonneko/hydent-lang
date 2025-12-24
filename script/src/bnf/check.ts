import { readFile } from "../utility";
import { parseBNF } from "./parse";
import { BNF_Alternative, BNF_AST } from "./type";

type Checker = (ctx: {
    bnf_file: string,
    bnf_ast: BNF_AST
}) => boolean | Promise<boolean>;

export const checks: {
    [key: string]: Checker;
} = {
    "bnf::bnf_exist": async (ctx) => {
        const bnf_file = await readFile("/assets/grammer.bnf");
        if (bnf_file !== null) {
            ctx.bnf_file = bnf_file;
            return true;
        }
        return false;
    },
    "bnf::bnf_is_valid": (ctx) => {
        const bnf_ast = parseBNF(ctx.bnf_file);
        if (bnf_ast !== null) {
            ctx.bnf_ast = bnf_ast;
            return true;
        }
        return false;
    },
    "bnf::all_non_terminal_character_defined": (ctx) => {
        const nonTerminals = new Set<string>();
        const terminals = new Set<string>();

        const collectRighty = (BnfAlternative: BNF_Alternative) => {
            BnfAlternative.terms.forEach(term => {
                term.factors.forEach(factor => {
                    switch (factor.node_type) {
                        case "terminal_string":
                            terminals.add(factor.terminal_string);
                            break;
                        case "character":
                            nonTerminals.add(factor.character);
                            break;
                        case "group":
                        case "repeat":
                        case "option":
                            collectRighty(factor.alternative);
                            break;
                    }
                })
            })
        }

        ctx.bnf_ast.rules.forEach(rule => {
            collectRighty(rule.alternative);
        })

        const definedNonTerminal = ctx.bnf_ast.rules.map(rule => rule.name);

        nonTerminals.forEach(nonTerminal => ctx.bnf_ast.rules.)


        return false;
    }
}

