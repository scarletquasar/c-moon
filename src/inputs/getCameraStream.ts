import { randomUUID } from 'crypto';
import NodeWebcam from 'node-webcam';
import { PassThrough } from 'stream';

const opts = {
    width: 1280,
    height: 720,
    quality: 100,
    frames: 60,
    delay: 0,
    saveShots: true,
    output: 'jpeg',
    device: false,
    callbackReturn: 'location',
    verbose: true,
} as const;

async function getCameraStream(): Promise<NodeJS.WritableStream> {
    var webcam = NodeWebcam.create(opts);
    await new Promise<void>((resolve, reject) => {
        const interval = setInterval(() => {
            webcam.capture(`${randomUUID().replaceAll('-', '')}`, function (err: any, data: any) {
                if (err) {
                    clearInterval(interval);
                    reject(err);
                    return;
                }
            });
        }, 1000 / opts.frames);

        setTimeout(() => {
            clearInterval(interval);
            resolve();
        }, 3000);
    });
    const bufferArray = webcam.shots.map((shot) => shot.data);
    const stream = new PassThrough();
    bufferArray.forEach((buffer) => stream.write(buffer));
    stream.end();
    return stream;
}

export { getCameraStream };
