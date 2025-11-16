/**
 * Convert string to upper camel case.
 * @param {string} str 
 * @returns 
 */
export function toUpperCamelCase(str) {
    return str.replace(/(?:^|-|_|\s+)(\w)/g, (_, c) => c.toUpperCase());
}
