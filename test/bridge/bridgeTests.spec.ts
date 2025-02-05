import { createAsyncBridge } from '../../src/bridge/createAsyncBridge';
import { copyFileSync, existsSync, readFileSync } from 'fs';
import { describe, expect, test, beforeAll, afterAll } from '@jest/globals';
import { mkdirSync } from 'fs';
import { join } from 'path';
import { exec } from 'child_process';
import { promises as fsPromises } from 'fs';

beforeAll(async () => {
    await new Promise<void>((resolve, reject) => {
        exec('cargo build --bin lib', { cwd: './lib' }, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve();
            }
        });
    });

    const tempDir = join(__dirname, '.temp');
    if (!existsSync(tempDir)) {
        mkdirSync(tempDir, { recursive: true });
    }

    const binaryName = process.platform === 'win32' ? 'lib.exe' : 'lib';
    const sourcePath = join(__dirname, '../../lib/target/debug', binaryName);
    copyFileSync(sourcePath, join(tempDir, binaryName));
});

afterAll(async () => {
    const tempDir = join(__dirname, '.temp');
    if (existsSync(tempDir)) {
        await fsPromises.rm(tempDir, { recursive: true, force: true });
    }
});

describe('Bridge testing: executing the application commands through the Rust bridge', () => {
    it('Should execute the IO action correctly', async () => {});

    it('Should execute the API action correctly', async () => {});

    it('Should execute the Download YouTube action correctly', async () => {});

    it('Should execute the Download Bluesky action correctly', async () => {});

    it('Should execute the Database action correctly', async () => {});

    it('Should execute the Audio Response action correctly', async () => {});

    it('Should execute the Text Response action correctly', async () => {});

    it('Should execute the Image Response action correctly', async () => {});

    it('Should execute the Passwords action correctly', async () => {});

    it('Should execute the Midday AI action correctly', async () => {});

    it('Should execute the Wayback Archiving action correctly', async () => {});

    it('Should handle unknown actions correctly', async () => {});
});
