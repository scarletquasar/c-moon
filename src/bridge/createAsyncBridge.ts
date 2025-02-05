import { exec, spawn } from 'node:child_process';
import { BRIDGE_LOCATION, BRIDGE_TTL } from '../consts';
import { join } from 'node:path';

async function createAsyncBridge(
    action: string,
    binaryPath = BRIDGE_LOCATION,
    cwd = './',
): Promise<string> {
    return new Promise((resolve, reject) => {
        const timeout = setTimeout(() => {
            reject(new Error('Operation timed out'));
        }, BRIDGE_TTL);

        exec(`${binaryPath} ${action}`, { cwd }, (error, stdout, _) => {
            clearTimeout(timeout);
            if (error) {
                return reject(error.message);
            }
            resolve(stdout);
        });
    });
}

export { createAsyncBridge };
