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
    try {
        execSync(
            `cd ../ && cargo run build ./tests/fixture/${parentPath}/${name}.hyt --out ./tests/fixture/${parentPath}/new-${name}-ast.json --emit ast --verbose`,
            { encoding: "utf-8", stdio: "ignore" }
        );

        const now = readFileSync(`../tests/fixture/${parentPath}/new-${name}-ast.json`);
        const old = readFileSync(`../tests/fixture/${parentPath}/${name}-ast.json`);

        execSync(`rm ../tests/fixture/${parentPath}/new-${name}-ast.json`, { encoding: "utf-8", stdio: "ignore"});

        if (now.toString() !== old.toString()) {
            console.error(`❌ ASTs in ./tests/fixture/${parentPath}/${name}.hyt don't match.`);
            return;
        }

        console.log(`✅ ASTs in ./tests/fixture/${parentPath}/${name}.hyt match.`);
    } catch (_) {
        console.error(`❌ No comparation target file detected: ./tests/fixture/${parentPath}/${name}.hyt`);
        return;
    }
});
