import { execFileSync, spawnSync } from "node:child_process";
import {
    existsSync,
    mkdirSync,
    readFileSync,
    readdirSync,
    rmSync,
    writeFileSync,
} from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { isDeepStrictEqual } from "node:util";

const scriptRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repositoryRoot = resolve(scriptRoot, "..");
const fixtureRoot = resolve(repositoryRoot, "tests/fixture/basic");
const actualRoot = resolve(repositoryRoot, "out/compiler-test-actuals");
const compiler = resolve(repositoryRoot, "target/debug/hydent_lang_compiler");
const updateSnapshots = process.argv.includes("--update");

function listSourceFiles(directory: string): string[] {
    return readdirSync(directory, { withFileTypes: true })
        .flatMap((entry) => {
            const path = join(directory, entry.name);
            if (entry.isDirectory()) return listSourceFiles(path);
            if (entry.isFile() && entry.name.endsWith(".hyt") && !entry.name.startsWith("_")) {
                return [path];
            }
            return [];
        })
        .sort((a, b) => a.localeCompare(b));
}

function collectIdentifierValues(value: unknown, into = new Set<string>()): Set<string> {
    if (Array.isArray(value)) {
        for (const item of value) collectIdentifierValues(item, into);
        return into;
    }

    if (value !== null && typeof value === "object") {
        const node = value as Record<string, unknown>;
        if (node.type === "Identifier" && typeof node.value === "string") {
            into.add(node.value);
        }
        for (const child of Object.values(node)) collectIdentifierValues(child, into);
    }

    return into;
}

function collectRawDeclarations(value: unknown, into: Record<string, unknown>[] = []): Record<string, unknown>[] {
    if (Array.isArray(value)) {
        for (const item of value) collectRawDeclarations(item, into);
    } else if (value !== null && typeof value === "object") {
        const node = value as Record<string, unknown>;
        if (node.type === "RawDeclaration") into.push(node);
        for (const child of Object.values(node)) collectRawDeclarations(child, into);
    }
    return into;
}

function skipSourceTrivia(text: string, start: number): number | null {
    let index = start;
    while (index < text.length) {
        if (/[ \t\r\n]/.test(text[index])) {
            index += 1;
            continue;
        }
        if (text.startsWith("//", index) && text[index + 2] !== "/") {
            const newline = text.indexOf("\n", index + 2);
            index = newline < 0 ? text.length : newline + 1;
            continue;
        }
        if (text.startsWith("/*", index)) {
            let depth = 1;
            index += 2;
            while (index < text.length && depth > 0) {
                if (text.startsWith("/*", index)) {
                    depth += 1;
                    index += 2;
                } else if (text.startsWith("*/", index)) {
                    depth -= 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            if (depth !== 0) return null;
            continue;
        }
        break;
    }
    return index;
}

function validateAst(sourcePath: string, source: string, ast: unknown): string[] {
    const errors: string[] = [];
    if (ast === null || typeof ast !== "object" || Array.isArray(ast)) {
        return ["root is not an AST object"];
    }

    const module = ast as Record<string, unknown>;
    if (module.type !== "Module") errors.push("root node is not Module");
    if (!Array.isArray(module.TopLevelStatement)) {
        errors.push("Module.TopLevelStatement is not an array");
        return errors;
    }
    if (module.TopLevelStatement.length === 0) {
        errors.push("non-empty source produced an empty Module AST");
    }

    const identifiers = collectIdentifierValues(ast);
    const declarations = [...source.matchAll(/\b(?:fn|module|struct|class|enum|protocol|type|const|static)\s+([A-Za-z_][A-Za-z_0-9]*)/g)]
        .map((match) => match[1]);
    for (const declaration of declarations) {
        if (!identifiers.has(declaration)) {
            errors.push(`AST is missing declaration name ${JSON.stringify(declaration)}`);
        }
    }

    for (const rawDeclaration of collectRawDeclarations(ast)) {
        const rawSource = rawDeclaration.source;
        const syntax = rawDeclaration.syntax;
        if (typeof rawSource !== "string" || !source.includes(rawSource)) {
            errors.push("RawDeclaration source does not match a source-file slice");
            continue;
        }
        if (!Array.isArray(syntax) || syntax.length === 0) {
            errors.push("RawDeclaration has no syntax tokens");
            continue;
        }

        let cursor = 0;
        let mismatch: string | undefined;
        for (const [index, item] of syntax.entries()) {
            if (
                item === null
                || typeof item !== "object"
                || typeof (item as Record<string, unknown>).value !== "string"
            ) {
                mismatch = `invalid syntax token at index ${index}`;
                break;
            }
            const value = (item as Record<string, string>).value;
            const tokenStart = skipSourceTrivia(rawSource, cursor);
            if (
                tokenStart === null
                || !rawSource.startsWith(value, tokenStart)
                || (index === 0 && tokenStart !== 0)
            ) {
                const offset = tokenStart ?? cursor;
                mismatch = `expected token ${index} ${JSON.stringify(value)} at source offset ${offset}, found ${JSON.stringify(rawSource.slice(offset, offset + 80))}`;
                break;
            }
            cursor = tokenStart + value.length;
        }
        const suffixStart = skipSourceTrivia(rawSource, cursor);
        if (!mismatch && suffixStart !== rawSource.length) {
            const offset = suffixStart ?? cursor;
            mismatch = `unmatched source suffix ${JSON.stringify(rawSource.slice(offset, offset + 80))}`;
        }
        if (mismatch) {
            errors.push(`RawDeclaration syntax tokens do not match the source slice in order: ${mismatch}`);
        }
    }

    if (errors.length > 0) {
        errors.unshift(relative(repositoryRoot, sourcePath));
    }
    return errors;
}

function githubError(title: string, message: string) {
    const escaped = message
        .replace(/%/g, "%25")
        .replace(/\r/g, "%0D")
        .replace(/\n/g, "%0A");
    console.error(`::error title=${title}::${escaped}`);
}

function main() {
    rmSync(actualRoot, { recursive: true, force: true });
    mkdirSync(actualRoot, { recursive: true });

    console.log("Generating parser and AST sources...");
    execFileSync("npm", ["start"], { cwd: scriptRoot, stdio: "inherit" });

    console.log("Building compiler...");
    execFileSync("cargo", ["build", "--quiet"], { cwd: repositoryRoot, stdio: "inherit" });

    const sourceFiles = listSourceFiles(fixtureRoot);
    const results: Array<Record<string, unknown>> = [];
    const failures: string[] = [];
    let failed = false;

    for (const sourcePath of sourceFiles) {
        const relativePath = relative(repositoryRoot, sourcePath);
        const snapshotPath = sourcePath.replace(/\.hyt$/, "-ast.json");
        const outputPath = join(actualRoot, relative(fixtureRoot, sourcePath).replace(/\.hyt$/, ".json"));
        mkdirSync(dirname(outputPath), { recursive: true });

        const run = spawnSync(
            compiler,
            ["--verbose", "build", sourcePath, "--out", outputPath, "--emit", "ast"],
            { cwd: repositoryRoot, encoding: "utf8", maxBuffer: 16 * 1024 * 1024 },
        );
        const stdout = run.stdout ?? "";
        const stderr = run.stderr ?? "";
        const record: Record<string, unknown> = {
            source: relativePath,
            exitCode: run.status,
            signal: run.signal,
            stdout,
            stderr,
            actualAst: relative(repositoryRoot, outputPath),
        };
        const errors: string[] = [];

        if (run.error) errors.push(`compiler failed to start: ${run.error.message}`);
        if (run.status !== 0) {
            errors.push(`compiler exited with status ${run.status ?? run.signal ?? "unknown"}`);
        }

        let actualAst: unknown;
        if (!existsSync(outputPath)) {
            errors.push("compiler did not produce an AST JSON file");
        } else {
            try {
                actualAst = JSON.parse(readFileSync(outputPath, "utf8"));
                errors.push(...validateAst(sourcePath, readFileSync(sourcePath, "utf8"), actualAst));
            } catch (error) {
                errors.push(`compiler output is not valid JSON: ${(error as Error).message}`);
            }
        }

        if (actualAst !== undefined && updateSnapshots && run.status === 0 && errors.length === 0) {
            writeFileSync(snapshotPath, `${JSON.stringify(actualAst, null, 2)}\n`, "utf8");
            console.log(`Updated ${relative(repositoryRoot, snapshotPath)}`);
        } else if (actualAst !== undefined && run.status === 0 && errors.length === 0) {
            if (!existsSync(snapshotPath)) {
                errors.push(`missing expected snapshot ${relative(repositoryRoot, snapshotPath)}`);
            } else {
                try {
                    const expectedAst = JSON.parse(readFileSync(snapshotPath, "utf8"));
                    if (!isDeepStrictEqual(actualAst, expectedAst)) {
                        errors.push(`AST differs from ${relative(repositoryRoot, snapshotPath)}`);
                    }
                } catch (error) {
                    errors.push(`expected snapshot is not valid JSON: ${(error as Error).message}`);
                }
            }
        }

        record.errors = errors;
        results.push(record);
        if (errors.length > 0) {
            failed = true;
            const astPreview = actualAst === undefined
                ? "AST JSON unavailable"
                : JSON.stringify(actualAst).slice(0, 1200);
            failures.push(
                `${relative(fixtureRoot, sourcePath)}: ${errors.join("; ")}; AST JSON: ${astPreview}`,
            );
            if (stdout.trim().length > 0) console.error(stdout.trim());
            if (stderr.trim().length > 0) console.error(stderr.trim());
        } else {
            console.log(`✅ ${relative(fixtureRoot, sourcePath)}`);
        }
    }

    writeFileSync(join(actualRoot, "results.json"), `${JSON.stringify(results, null, 2)}\n`, "utf8");
    console.log(`AST outputs and diagnostics: ${relative(repositoryRoot, actualRoot)}`);

    if (failed) {
        for (const failure of failures) {
            // Keep each annotation short and fixture-specific. A single aggregate
            // annotation is truncated by GitHub, hiding failures near the end.
            githubError("Compiler fixture snapshot failure", failure.slice(0, 3000));
        }
        throw new Error("Compiler fixture snapshot tests failed.");
    }
    console.log("\n✅ All compiler fixture snapshot tests passed.");
}

main();
