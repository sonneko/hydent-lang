import { execSync } from "child_process";
import { readdirSync, readFileSync, writeFileSync } from "fs";

const sourceFiles = readdirSync('../tests/fixture/', {
    withFileTypes: true,
    recursive: true
}).sort((a, b) => a.name.localeCompare(b.name));

let isOk = true;

sourceFiles.filter(file => file.isFile()).forEach((file) => {
    const isCiMode = process.argv[2] === "ci";

    if (file.name.split(".")[1] !== "hyt") {
        return;
    }
    const parentPath = file.parentPath.split("/").filter((_, i) => i > 2);
    const name = file.name.split(".")[0];

    if (name.startsWith("_")) {
        console.log(`Ignore test detected in ./tests/fixture/${parentPath}/${name}.hyt`);
        return;
    }

    try {
        execSync(
            `cd ../ && cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./tests/fixture/${parentPath}/new-${name}-ast.json --emit ast --verbose`,
            { encoding: "utf-8", stdio: isCiMode ? "inherit" : "pipe" }
        );
    } catch(e) {
        console.log(`❌ Parse failed. Run: cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./tests/fixture/${parentPath}/new-${name}-ast.json --emit ast --verbose`);
        const err = (e as any).stdout.toString() + "\n\n" + (e as any).stderr.toString();
        writeFileSync(`../tests/fixture/${parentPath}/${name}-error.log.txt`, err, { encoding: "utf-8" });
        isOk = false;
        return;
    }

    try {
        const now = readFileSync(`../tests/fixture/${parentPath}/new-${name}-ast.json`);
        const old = readFileSync(`../tests/fixture/${parentPath}/${name}-ast.json`);

        execSync(`rm ../tests/fixture/${parentPath}/new-${name}-ast.json`, { encoding: "utf-8", stdio: "ignore"});

        if (now.toString() !== old.toString()) {
            console.error(`❌ ASTs in ./tests/fixture/${parentPath}/${name}.hyt don't match.`);
            return;
        }

        console.log(`✅ ASTs in ./tests/fixture/${parentPath}/${name}.hyt match.`);

        return;
    } catch (_) {
        console.error(`❌ No comparation target file detected: ./tests/fixture/${parentPath}/${name}.hyt`);
        execSync(`mv ../tests/fixture/${parentPath}/new-${name}-ast.json ../tests/fixture/${parentPath}/${name}-ast.json`, { encoding: "utf-8", stdio: "ignore" });
        isOk = false;
        return;
    }
});

if (!isOk) {
    throw new Error("❌ Failed the tests.");
}

console.log("\n\n✅ All tests passed.")
