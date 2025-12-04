import { check } from "./check.mjs";

const args = process.argv.slice(2);

if (args.length === 0) {
    console.error("No arguments provided. `build`, `full_check` or `check` is required.");
    process.exit(1);
} else {
    const cmd_name = args[0];
    if (cmd_name === "build") {
        console.log("Unimplemented command: build");
        // build();
    } else if (cmd_name === "full_check") {
        console.log("Unimplemented command: full_check");
        // run();
    } else if (cmd_name === "check") {
        check();
    } else {
        console.error("Unknown command: " + cmd_name);
        process.exit(1);
    }
}