import { createAsyncBridge } from '../../src/bridge/createAsyncBridge';
import { copyFileSync } from 'fs';
import { exec } from 'child_process';

before((done) => {
    exec('sh tests/bridge/bridgeTestsSetup.sh', (error, stdout, _) => {
        if (error) {
            return done(error);
        }
        console.log(stdout);
        done();
    });
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
