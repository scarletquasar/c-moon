import Fastify from 'fastify';

export async function createHttpServer() {
    const fastify = Fastify({ logger: true });

    // Route to send a chat message and receive a response
    fastify.post('/chat', async (request, reply) => {
        const { message } = request.body as { message: string };
        const responseMessage = `Received your message: ${message}`;
        return { response: responseMessage };
    });

    // Route to serve the dashboard page
    fastify.get('/dashboard', async (request, reply) => {
        const dashboardHtml = `
			<html>
				<head>
					<title>Dashboard</title>
				</head>
				<body>
					<h1>Application Logs</h1>
					<ul id="logs"></ul>
					<script>
						const eventSource = new EventSource('/logs');
						eventSource.onmessage = function(event) {
							const logList = document.getElementById('logs');
							const newLog = document.createElement('li');
							newLog.textContent = event.data;
							logList.appendChild(newLog);
						};
					</script>
				</body>
			</html>
		`;
        reply.type('text/html').send(dashboardHtml);
    });

    try {
        await fastify.listen({ port: 3000, host: '0.0.0.0' });
        console.log('Server is running at http://localhost:3000');
    } catch (err) {
        fastify.log.error(err);
        process.exit(1);
    }
}
