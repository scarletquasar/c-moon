import { getCameraStream } from './getCameraStream';
import { getMicrophoneStream } from './getMicrophoneStream';

async function recursiveAwaitForInputs() {
    const microphonePromise = getMicrophoneStream(3000);
    const cameraPromise = getCameraStream(3000);

    const [microphoneStream, cameraStream] = await Promise.all([microphonePromise, cameraPromise]);
}
