import { spawn } from 'node:child_process';
import { BRIDGE_LOCATION, BRIDGE_TTL } from '../consts';

async function createAsyncBridge(
    bridgeLocation: string,
    command: string,
    json: string,
): Promise<string> {
    const bridgeSpawn = spawn(bridgeLocation, [command, json]);

    return await new Promise<string>((resolve, reject) => {
        const buffers: Buffer[] = [];

        let timeout = setTimeout(() => {
            cleanup();
            reject(
                new Error(
                    'The time to live of the bridge connection reached the end without response',
                ),
            );
        }, BRIDGE_TTL);

        const cleanup = () => {
            clearTimeout(timeout);
            bridgeSpawn.stdout?.off('data', onData);
            bridgeSpawn.off('error', onError);
            bridgeSpawn.off('close', onClose);
        };

        const onData = (data: Buffer) => {
            console.log('Data received:', data.toString('utf-8'));
            buffers.push(data);
        };

        const onError = (err: Error) => {
            console.error('Error received:', err);
            buffers.push(Buffer.from(err.message));
            cleanup();
            reject(err);
        };

        const onClose = (code: number) => {
            cleanup();
            if (code !== 0) {
                const lastMessage = buffers.length
                    ? buffers[buffers.length - 1].toString('utf-8')
                    : 'No error message received';
                console.error(`Bridge process closed with code ${code}: ${lastMessage}`);
                reject(
                    new Error(`The bridge connection returned error code ${code}: ${lastMessage}`),
                );
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
