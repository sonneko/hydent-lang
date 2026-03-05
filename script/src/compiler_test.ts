import { execSync } from "child_process";
import { readdirSync, readFileSync } from "fs";

const sourceFiles = readdirSync('../tests/fixture/', {
    withFileTypes: true,
    recursive: true
});

sourceFiles.filter(file => file.isFile()).forEach((file) => {
    if (file.name.split(".")[1] !== "hyt") {
        return;
    }
    const parentPath = file.parentPath.split("/").filter((_, i) => i > 2);
    const name = file.name.split(".")[0];
    execSync(`mkdir -p ../out/snapshots/${parentPath}`);
    execSync(
        `cd ../ && cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./out/snapshots/${parentPath}/${name}-ast.json --emit ast --verbose`,
        { encoding: "utf-8", stdio: "ignore" }
    );
    try {
        execSync(
            `git checkout gh-pages:snapshots/${parentPath}/${name}-ast.json > ../out/snapshots/${parentPath}/old-${name}-ast.json`,
        );
        const now = readFileSync(`../out/snapshots/${parentPath}/${name}-ast.json`);
        const old = readFileSync(`../out/snapshots/${parentPath}/old-${name}-ast.json`);

        if (now.toString() !== old.toString()) {
            throw new Error(`❌ ASTs in ./tests/fixture/${parentPath}/${name}.hyt don't match.`);
        }

        execSync(`rm ../out/snapshots/${parentPath}/old-${name}-ast.json`);
        console.log(`✅ ASTs in ./tests/fixture/${parentPath}/${name}.hyt match.`);
    } catch (_) {
        execSync(`rm ../out/snapshots/${parentPath}/old-${name}-ast.json`);
        console.log(`👀 New fixture detected: ./tests/fixture/${parentPath}/${name}.hyt`);
    }
});
