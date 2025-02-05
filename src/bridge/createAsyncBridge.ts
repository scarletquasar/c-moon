import { spawn } from 'node:child_process';
import { BRIDGE_LOCATION, BRIDGE_TTL } from '../consts';

async function createAsyncBridge(command: string, json: string): Promise<string> {
    const bridgeSpawn = spawn(BRIDGE_LOCATION, [command, json]);

    return await new Promise<string>((resolve, reject) => {
        const buffers: Buffer[] = [];

        let timeout = setTimeout(() => {
            cleanup();
            reject(new Error('Timeout waiting for process exit'));
        }, BRIDGE_TTL);

        const cleanup = () => {
            clearTimeout(timeout);
            bridgeSpawn.stdout?.off('data', onData);
            bridgeSpawn.off('error', onError);
            bridgeSpawn.off('close', onClose);
        };

        const onData = (data: Buffer) => {
            buffers.push(data);
        };

        const onError = (err: Error) => {
            cleanup();
            reject(err);
        };

        const onClose = (code: number) => {
            cleanup();
            if (code !== 0) {
                reject(new Error(`Process exited with code ${code}`));
            } else {
                resolve(Buffer.concat(buffers).toString('utf-8'));
            }
        };

        bridgeSpawn.stdout?.on('data', onData);
        bridgeSpawn.on('error', onError);
        bridgeSpawn.on('close', onClose);
    });
}

export { createAsyncBridge };
