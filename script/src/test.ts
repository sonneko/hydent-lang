import "./parser_gen/tests";

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
