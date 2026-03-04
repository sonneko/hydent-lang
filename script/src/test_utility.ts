
export function describe(name: string, fn: () => void) {
    console.log(`\n=== Testing: ${name} ===`);
    fn();
}

export function it(name: string, fn: () => void) {
    try {
        fn();
        console.log(`✅ ${name}`);
    } catch (e) {
        console.error(`❌ ${name}`);
        console.error(e);
    }
}

export function assert(condition: boolean, message: string) {
    if (!condition) throw new Error(message);
}

export function or(a: () => void, b: () => void) {
    try {
        a();
    } catch (e) {
        b();
    }
}

export function match(received: any) {
    return {
        toBe: function (expected: any) { assert(received === expected, `Expected ${expected} but got ${received}`)} ,
        toBeDefined: function () {assert(received !== undefined && received !== null, `Expected value to be defined`)},
        toContain: function (expected: string) {
            const str = String(received);
            assert(str.includes(expected), `Expected ${str} to contain ${expected}`);
        },
        toContainEqual: function (expectedObj: any) {
            const found = (received as any[]).some(item =>
                Object.keys(expectedObj).every(key => item[key] === expectedObj[key])
            );
            assert(found, `Expected array to contain object matching ${JSON.stringify(expectedObj)}`);
        },
        toThrowError: function (messagePart: string) {
            let errorThrown = false;
            try {
                received();
            } catch (e: any) {
                errorThrown = true;
                assert(e.message.includes(messagePart), `Expected error message to include "${messagePart}", but got "${e.message}"`);
            }
            assert(errorThrown, "Expected error to be thrown but none was");
        },
    }
};
