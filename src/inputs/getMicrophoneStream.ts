import Mic from 'node-microphone';

async function getMicrophoneStream(timeChunk: number) {
    let mic = new Mic();
    let micStream = mic.startRecording();
    await new Promise((resolve) => setTimeout(resolve, timeChunk));
    mic.stopRecording();
    return micStream;
}

export { getMicrophoneStream };
