import { readFile as readFileRaw } from "node:fs/promises";

export async function readFile(path: string): Promise<string | null> {
    try {
        return readFileRaw(process.cwd() + path, {
            encoding: "utf8",
        });
    } catch (err) {
        return null;
    }
}
