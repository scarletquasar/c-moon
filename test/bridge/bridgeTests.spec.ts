import { createAsyncBridge } from '../../src/bridge/createAsyncBridge';
import { copyFileSync, existsSync, readFileSync } from 'fs';
import { describe, expect, test, beforeAll, afterAll } from '@jest/globals';
import { mkdirSync } from 'fs';
import { join } from 'path';
import { exec } from 'child_process';
import { promises as fsPromises } from 'fs';

beforeAll(async () => {
    await new Promise<void>((resolve, reject) => {
        exec('cargo build --bin lib', { cwd: './lib' }, (error, _, __) => {
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
    const getBinaryPath = (): string => {
        const binaryName = process.platform === 'win32' ? 'lib.exe' : 'lib';
        return join(__dirname, '.temp', binaryName);
    };

    const runCommand = (action: string) =>
        createAsyncBridge(action, getBinaryPath(), join(__dirname, '.temp'));

    it('Should execute the IO action correctly for creating a file', async () => {
        const args = JSON.stringify({
            method: 'create',
            content: 'hello world',
            type: 'file',
            name: 'temp-create.txt',
        });

        const stdout = await runCommand(`io ${JSON.stringify(args)}`);
        expect(stdout).toContain(`[io] Executing function with argument: ${args}`);
        expect(stdout).toContain('[io] Command executed with success, result: {}');

        const filePath = join(__dirname, '.temp', 'temp-create.txt');
        expect(existsSync(filePath)).toBe(true);
    });

    it('Should execute the IO action correctly for updating a file', async () => {
        const args = JSON.stringify({
            method: 'create',
            content: 'hello world',
            type: 'file',
            name: 'temp-create.txt',
        });

        const stdout = await runCommand(`io ${JSON.stringify(args)}`);
        expect(stdout).toContain(`[io] Executing function with argument: ${args}`);
        expect(stdout).toContain('[io] Command executed with success, result: {}');

        const filePath = join(__dirname, '.temp', 'temp-create.txt');
        expect(existsSync(filePath)).toBe(true);

        const editArgs = JSON.stringify({
            method: 'update',
            content: 'hello world1',
            type: 'file',
            name: 'temp-create.txt',
        });

        const stdoutEdit = await runCommand(`io ${JSON.stringify(editArgs)}`);
        expect(stdoutEdit).toContain(`[io] Executing function with argument: ${editArgs}`);
        expect(stdoutEdit).toContain('[io] Command executed with success, result: {}');

        const editedFileRaw = readFileSync('./test/bridge/.temp/temp-create.txt', 'utf-8');
        const editedFileContent = editedFileRaw.toString();
        expect(editedFileContent).toBe('hello world1');
    });

    it('Should execute the API action correctly', async () => {
        const stdout = await runCommand('api');
        expect(stdout).toContain('[api] Executing function with argument: ');
        //expect(stdout).toContain('[api] Command executed with success, result: {}');
    });

    it('Should execute the Download YouTube action correctly', async () => {
        const stdout = await runCommand('download-youtube');
        expect(stdout).toContain('[download-youtube] Executing function with argument: ');
        //expect(stdout).toContain('[download-youtube] Command executed with success, result: {}');
    });

    it('Should execute the Download Bluesky action correctly', async () => {
        const stdout = await runCommand('download-bluesky');
        expect(stdout).toContain('[download-bluesky] Executing function with argument: ');
        //expect(stdout).toContain('[download-bluesky] Command executed with success, result: {}');
    });

    it('Should execute the Database action correctly', async () => {
        const stdout = await runCommand('database');
        expect(stdout).toContain('[database] Executing function with argument: ');
        //expect(stdout).toContain('[database] Command executed with success, result: {}');
    });

    it('Should execute the Audio Response action correctly', async () => {
        const stdout = await runCommand('audio-response');
        expect(stdout).toContain('[audio-response] Executing function with argument: ');
        //expect(stdout).toContain('[audio-response] Command executed with success, result: {}');
    });

    it('Should execute the Text Response action correctly', async () => {
        const stdout = await runCommand('text-response');
        expect(stdout).toContain('[text-response] Executing function with argument: ');
        //expect(stdout).toContain('[text-response] Command executed with success, result: {}');
    });

    it('Should execute the Image Response action correctly', async () => {
        const stdout = await runCommand('image-response');
        expect(stdout).toContain('[image-response] Executing function with argument: ');
        //expect(stdout).toContain('[image-response] Command executed with success, result: {}');
    });

    it('Should execute the Passwords action correctly', async () => {
        const stdout = await runCommand('passwords');
        expect(stdout).toContain('[passwords] Executing function with argument: ');
        //expect(stdout).toContain('[passwords] Command executed with success, result: {}');
    });

    it('Should execute the Midday AI action correctly', async () => {
        const stdout = await runCommand('midday-ai');
        expect(stdout).toContain('[midday-ai] Executing function with argument: ');
        //expect(stdout).toContain('[midday-ai] Command executed with success, result: {}');
    });

    it('Should execute the Wayback Archiving action correctly', async () => {
        const stdout = await runCommand('wayback-archiving');
        expect(stdout).toContain('[wayback-archiving] Executing function with argument: ');
        //expect(stdout).toContain('[wayback-archiving] Command executed with success, result: {}');
    });

    it('Should handle unknown actions correctly', async () => {
        const stdout = await runCommand('unknown');
        expect(stdout).toContain('[unknown] Executing function with argument: ');
    });
});
