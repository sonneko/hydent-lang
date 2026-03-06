import { execSync } from "child_process";
import { readdirSync, readFileSync } from "fs";

const sourceFiles = readdirSync('../tests/fixture/', {
    withFileTypes: true,
    recursive: true
});

const ok = sourceFiles.filter(file => file.isFile()).every((file) => {
    const isCiMode = process.argv[2] === "ci";

    if (file.name.split(".")[1] !== "hyt") {
        return true;
    }
    const parentPath = file.parentPath.split("/").filter((_, i) => i > 2);
    const name = file.name.split(".")[0];

    if (name.startsWith("_")) {
        console.log(`Ignore test detected in ./tests/fixture/${parentPath}/${name}.hyt`);
        return true;
    }

    try {
        execSync(
            `cd ../ && cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./tests/fixture/${parentPath}/new-${name}-ast.json --emit ast --verbose --should-success`,
            { encoding: "utf-8", stdio: isCiMode ? "inherit" : "ignore" }
        );
    } catch(e) {
        console.log(`❌ Parse failed. Run: cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./tests/fixture/${parentPath}/new-${name}-ast.json --emit ast --verbose --should-success`)
        return false;
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

        return true;
    } catch (_) {
        console.error(`❌ No comparation target file detected: ./tests/fixture/${parentPath}/${name}.hyt`);
        execSync(`mv ../tests/fixture/${parentPath}/new-${name}-ast.json ../tests/fixture/${parentPath}/${name}-ast.json`, { encoding: "utf-8", stdio: "ignore" });
        return false;
    }
});

if (!ok) {
    throw new Error("❌ Failed the tests.");
}

console.log("\n\n✅ All tests passed.")