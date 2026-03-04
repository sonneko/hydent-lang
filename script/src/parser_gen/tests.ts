import { describe, it, match, or } from '../test_utility';
import { analyze, Analyzer } from './analyze';
import { Grammar } from './ast';
import { parse } from "./parse";

describe('Parser (parse.ts)', () => {
    it('should parse Branch and Product rules correctly', () => {
        const src = `
            branch Expr {
                AddExpr
                SubExpr with "boxed"
            }
            product AddExpr {
                left: Expr
                "+"
                right: Expr
            }
            product SubExpr with "TerminalSub"
        `;
        const ast = parse(src);
        match(ast.length).toBe(3);
        match(ast[0].kind).toBe('Branch');
        match((ast[0] as any).variants[1].note).toBe('boxed');

        match(ast[1].kind).toBe('Product');
        match((ast[1] as any).members.length).toBe(3);

        match(ast[2].kind).toBe('Product');
        match((ast[2] as any).members[0].value).toBe('TerminalSub');
    });

    it('should parse repeated and optional elements correctly', () => {
        const src = `
            product ListRule {
                items: *Expr
                opt: ?Expr
            }
        `;
        const ast = parse(src);
        match(ast.length).toBe(1);
        const members = (ast[0] as any).members;
        match(members[0].type.modifier).toBe('List');
        match(members[1].type.modifier).toBe('Option');
    });
});

describe('Analyzer', () => {
    const mockTokenMap = {
        'a': 'TokenA',
        'b': 'TokenB',
        'c': 'TokenC',
    };

    describe('computeNullable', () => {
        it('should correctly identify nullable rules', () => {
            const grammar: Grammar = [
                // Empty -> (empty)
                { kind: 'Product', name: 'EmptyRule', members: [] },
                // HasTerminal -> "a"
                { kind: 'Product', name: 'HasTerminal', members: [{ kind: 'Terminal', value: 'a', note: '' }] },
                // OptionRule -> Identifier?
                {
                    kind: 'Product', name: 'OptionRule', members: [
                        { kind: 'Field', name: 'id', type: { name: 'SomeId', modifier: 'Option' }, note: '' }
                    ]
                },
                // BranchRule -> EmptyRule | HasTerminal
                {
                    kind: 'Branch', name: 'BranchNullable', variants: [
                        { name: 'EmptyRule', note: '' },
                        { name: 'HasTerminal', note: '' }
                    ]
                }
            ];

            const analyzer = new Analyzer(grammar, mockTokenMap);
            analyzer.computeNullable();

            match(analyzer.nullable.has('EmptyRule')).toBe(true);
            match(analyzer.nullable.has('HasTerminal')).toBe(false);
            match(analyzer.nullable.has('OptionRule')).toBe(true);
            match(analyzer.nullable.has('BranchNullable')).toBe(true);
        });
    });

    describe('computeFirst & computeFollow', () => {
        it('should compute FIRST and FOLLOW sets correctly for simple sequences', () => {
            const grammar: Grammar = [
                // S -> A "b"
                {
                    kind: 'Product', name: 'S', members: [
                        { kind: 'Field', name: 'a', type: { name: 'A', modifier: 'None' }, note: '' },
                        { kind: 'Terminal', value: 'b', note: '' }
                    ]
                },
                // A -> "a"
                { kind: 'Product', name: 'A', members: [{ kind: 'Terminal', value: 'a', note: '' }] }
            ];

            const analyzer = new Analyzer(grammar, mockTokenMap);
            analyzer.analyze();

            const firstS = (analyzer as any).firstSets.get('S');
            match([...firstS][0]).toBe('TokenA,TokenB');

            const followA = (analyzer as any).followSets.get('A');
            match([...followA][0]).toBe('TokenB');
        });

        it('should handle List modifiers by adding FIRST to FOLLOW', () => {
            const grammar: Grammar = [
                // ListRule -> A*
                {
                    kind: 'Product', name: 'ListRule', members: [
                        { kind: 'Field', name: 'a', type: { name: 'A', modifier: 'List' }, note: '' }
                    ]
                },
                // A -> "a"
                { kind: 'Product', name: 'A', members: [{ kind: 'Terminal', value: 'a', note: '' }] }
            ];

            const analyzer = new Analyzer(grammar, mockTokenMap);
            analyzer.analyze();

            const followA = (analyzer as any).followSets.get('A');
            match([...followA][0]).toBe('TokenA');
        });
    });

    describe('computeFirst & computeFollow complex cases', () => {
        it('should correctly compute FIRST and FOLLOW sets with mutually recursive and nullable rules', () => {
            const grammar: Grammar = [
                // S -> A B C
                { kind: 'Product', name: 'S', members: [
                    { kind: 'Field', name: 'a', type: { name: 'A', modifier: 'None' }, note: '' },
                    { kind: 'Field', name: 'b', type: { name: 'B', modifier: 'None' }, note: '' },
                    { kind: 'Field', name: 'c', type: { name: 'C', modifier: 'None' }, note: '' }
                ]},
                // A -> "a" | A_empty
                { kind: 'Branch', name: 'A', variants: [{ name: 'A_a', note: '' }, { name: 'A_empty', note: '' }] },
                { kind: 'Product', name: 'A_a', members: [{ kind: 'Terminal', value: 'a', note: '' }] },
                { kind: 'Product', name: 'A_empty', members: [] },
                // B -> "b" | B_empty
                { kind: 'Branch', name: 'B', variants: [{ name: 'B_b', note: '' }, { name: 'B_empty', note: '' }] },
                { kind: 'Product', name: 'B_b', members: [{ kind: 'Terminal', value: 'b', note: '' }] },
                { kind: 'Product', name: 'B_empty', members: [] },
                // C -> "c"
                { kind: 'Product', name: 'C', members: [{ kind: 'Terminal', value: 'c', note: '' }] },
            ];

            const analyzer = new Analyzer(grammar, mockTokenMap);
            analyzer.analyze();

            const firstS = (analyzer as any).firstSets.get('S');
            const firstSArray = [...firstS];
            match(firstSArray).toContain('TokenA,TokenB'); // A->a, B->b
            match(firstSArray).toContain('TokenA,TokenC'); // A->a, B->empty
            match(firstSArray).toContain('TokenB,TokenC'); // A->empty, B->b
            match(firstSArray).toContain('TokenC');        // A->empty, B->empty

            const followA = (analyzer as any).followSets.get('A');
            const followAArray = [...followA];
            match(followAArray).toContain('TokenB'); // when B is not empty
            match(followAArray).toContain('TokenC'); // when B is empty
        });
    });

    describe('analyzeBranchRule', () => {
        it('should differentiate branches into Peek0 and Peek1', () => {
            const grammar: Grammar = [
                // Branch -> V1 | V2 | V3
                {
                    kind: 'Branch', name: 'Branch', variants: [
                        { name: 'V1', note: '' },
                        { name: 'V2', note: '' },
                        { name: 'V3', note: '' }
                    ]
                },
                // V1 -> "a" "b"
                { kind: 'Product', name: 'V1', members: [{ kind: 'Terminal', value: 'a', note: '' }, { kind: 'Terminal', value: 'b', note: '' }] },
                // V2 -> "a" "c"
                { kind: 'Product', name: 'V2', members: [{ kind: 'Terminal', value: 'a', note: '' }, { kind: 'Terminal', value: 'c', note: '' }] },
                // V3 -> "b"
                { kind: 'Product', name: 'V3', members: [{ kind: 'Terminal', value: 'b', note: '' }] },
            ];

            const ir = analyze(grammar, mockTokenMap);
            const branchIR = ir.find(i => i.kind === 'branch') as any;

            match(branchIR).toBeDefined();

            match(branchIR.branchesJudgebleInPeek0).toContainEqual(
                { astTypeName: 'V3', firstTerminal: 'TokenB' }
            );

            match(branchIR.branchesJudgebleInPeek1).toContainEqual(
                { astTypeName: 'V1', firstTerminal: 'TokenA', secondTerminal: 'TokenB' }
            );
            match(branchIR.branchesJudgebleInPeek1).toContainEqual(
                { astTypeName: 'V2', firstTerminal: 'TokenA', secondTerminal: 'TokenC' }
            );
        });
    });

    describe('computeCycle (Boxed Types)', () => {
        it('should mark recursive fields as boxed to prevent infinite size in Rust', () => {
            const grammar: Grammar = [
                // Expr -> "a" | AddExpr
                {
                    kind: 'Branch', name: 'Expr', variants: [
                        { name: 'Term', note: '' },
                        { name: 'AddExpr', note: '' }
                    ]
                },
                // Term -> "a"
                { kind: 'Product', name: 'Term', members: [{ kind: 'Terminal', value: 'a', note: '' }] },
                // AddExpr -> Expr "b"
                {
                    kind: 'Product', name: 'AddExpr', members: [
                        { kind: 'Field', name: 'left', type: { name: 'Expr', modifier: 'None' }, note: '' },
                        { kind: 'Terminal', value: 'b', note: '' }
                    ]
                }
            ];

            const ir = analyze(grammar, mockTokenMap);

            const addExprIR = ir.find(i => i.astTypeName === 'AddExpr' && i.kind === 'product') as any;
            match(addExprIR).toBeDefined();

            const exprElement = addExprIR.elements.find((e: any) => e.astTypeName === 'Expr');

            const exprIR = ir.find(i => i.astTypeName === 'Expr' && i.kind === 'branch') as any;
            match(exprIR).toBeDefined();

            const addExprRef = [...exprIR.branchesJudgebleInPeek1][0];

            or(() => {
                match(exprElement.kind).toBe('boxed');
            }, () => {
                match(addExprRef.isBoxed).toBe(true);
            },);
        });
    });

    describe('checkLL2Confilict', () => {
        it('should throw an error if a Nullable rule has intersecting FIRST and FOLLOW sets', () => {
            // S -> A "a"
            // A -> "a" | (empty)
            const grammar: Grammar = [
                {
                    kind: 'Product', name: 'S', members: [
                        { kind: 'Field', name: 'a', type: { name: 'A', modifier: 'None' }, note: '' },
                        { kind: 'Terminal', value: 'a', note: '' }
                    ]
                },
                {
                    kind: 'Branch', name: 'A', variants: [
                        { name: 'A_a', note: '' },
                        { name: 'A_empty', note: '' }
                    ]
                },
                { kind: 'Product', name: 'A_a', members: [{ kind: 'Terminal', value: 'a', note: '' }] },
                { kind: 'Product', name: 'A_empty', members: [] },
            ];

            const analyzer = new Analyzer(grammar, mockTokenMap);

            match(() => {
                analyzer.analyze();
            }).toThrowError('Grammar conflict');
        });
    });
});
