import { exec as raw_exec } from "child_process";
import { readFile as raw_readFile } from "fs";
import { writeFile as raw_writeFile } from "fs";
import { readdir } from "fs";


/**
 * Executes a shell command and returns the output.
 * @param {string} cmd The command to execute.
 * @returns {Promise<{stdout: string, stderr: string}>} A promise that resolves with the standard output and standard error.
 */
export async function exec(cmd) {
    return new Promise((resolve, reject) => {
        raw_exec(cmd, (err, stdout, stderr) => {
            if (err) {
                reject(err);
            } else {
                resolve({ stdout, stderr });
            }
        })
    })
}

/**
 * Reads a file and returns its content as a string.
 * @param {string} path The path to the file.
 * @returns {Promise<string>} A promise that resolves with the file content.
 */
export async function readFile(path) {
    return new Promise((resolve, reject) => {
        raw_readFile(process.cwd() + path, (err, data) => {
            if (err) {
                reject(err);
            }
            resolve(data.toString());
        })
    });
}

/**
 * Reads all file names in a directory.
 * @param {string} dir_path The path to the directory.
 * @returns {Promise<string[]>} A promise that resolves with an array of file names.
 */
export async function readAllFilesInDir(dir_path) {
    return new Promise((resolve, reject) => {
        readdir(dir_path, (err, files) => {
            if (err) {
                reject(err)
            }
            resolve(files)
        })
    })
}

/**
 * Writes data to a file.
 * @param {string} path The path to the file.
 * @param {string} data The data to write.
 * @returns {Promise<void>} A promise that resolves when the file has been written.
 */
export async function writeFile(path, data) {
    return new Promise((resolve, reject) => {
        raw_writeFile(process.cwd() + path, data, (err) => {
            if (err) {
                reject(err);
            }
            resolve();
        })
    })
}


/**
 * Asserts that a condition is true, otherwise exits the process.
 * @param {boolean} condition The condition to assert.
 */
export function assert(condition) {
    if (!condition) {
        process.exit(1);
    }
}

/**
 * Panics the process.
 */
export function panic() {
    process.exit(1);
}
