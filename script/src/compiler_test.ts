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
            githubError(`Compiler fixture: ${relative(fixtureRoot, sourcePath)}`, errors.join("; "));
            if (stdout.trim().length > 0) console.error(stdout.trim());
            if (stderr.trim().length > 0) console.error(stderr.trim());
        } else {
            console.log(`✅ ${relative(fixtureRoot, sourcePath)}`);
        }
    }

    writeFileSync(join(actualRoot, "results.json"), `${JSON.stringify(results, null, 2)}\n`, "utf8");
    console.log(`AST outputs and diagnostics: ${relative(repositoryRoot, actualRoot)}`);

    if (failed) {
        throw new Error("Compiler fixture snapshot tests failed.");
    }
    console.log("\n✅ All compiler fixture snapshot tests passed.");
}

main();
