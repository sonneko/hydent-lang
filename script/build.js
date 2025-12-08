import { exec } from "./helpers/sys.js";

export function build() {
    exec("cargo build");
    
}