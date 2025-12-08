import { check } from "./check.js";
import { build } from "./build.js";

const args = process.argv.slice(2);

if (args.length === 0) {
    console.error("No arguments provided. `build` is required.");
    process.exit(1);
} else {
    const cmd_name = args[0];
    if (cmd_name === "build") {
        check();
        build();
    } else {
        console.error("Unknown command: " + cmd_name);
        process.exit(1);
    }
}
