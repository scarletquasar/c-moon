import { exec } from 'child_process';
import { copyFileSync, existsSync, mkdirSync } from 'fs';
import { join } from 'path';

async function setupAndRun() {
    await new Promise<void>((resolve, reject) => {
        exec('cargo build --bin lib', { cwd: './lib' }, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve();
            }
        });
    });

    const distDir = 'dist';
    if (!existsSync(distDir)) {
        mkdirSync(distDir, { recursive: true });
    }

    const binaryName = process.platform === 'win32' ? 'lib.exe' : 'lib';
    const sourcePath = join('./lib/target/debug', binaryName);
    copyFileSync(sourcePath, join(distDir, binaryName));
}

setupAndRun();
