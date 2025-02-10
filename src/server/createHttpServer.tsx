import Fastify from 'fastify';

export async function createHttpServer() {
    const fastify = Fastify({ logger: true });

    fastify.post('/chat', async (request, reply) => {
        const { message } = request.body as { message: string };
        const responseMessage = `Received your message: ${message}`;
        return { response: responseMessage };
    });

    try {
        await fastify.listen({ port: 3000, host: '0.0.0.0' });
        console.log('Server is running at http://localhost:3000');
    } catch (err) {
        fastify.log.error(err);
        process.exit(1);
    }
}
