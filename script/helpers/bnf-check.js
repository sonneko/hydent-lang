/**
 * Check all chars defined in rules.
 * @param {{ left: string, right: string }[] } rules 
 */
export function checkAllCharsDefined(rules) {
    const rights = rules.reduce((pre, curr) => {
        pre.push(...curr.right);
        return pre;
    }, []);
    const lefts = rules.reduce((pre, curr) => {
        pre.push(curr.left);
        return pre;
    }, []);
    const s1 = new Set(rights);
    const s2 = new Set(lefts);
    if (s1.size !== s2.size) return false;
    assert([...s1].every(v => {
        if (!s2.has(v)) {
            throw new Error(`undefined char: ${v}`)
        };
        return true;
    }));
}
