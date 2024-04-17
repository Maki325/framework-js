import './impl.js';
import { createServer } from 'http';
import Page from './component.js';
import { Readable } from 'stream';

async function main() {
  const server = createServer(async (req, res) => {
    if (req.url === '/favicon.ico') {
      res.end();
      return;
    }

    /** @type ReadableStreamDefaultController */
    let _controller;
    const rb = new ReadableStream({
      start(controller) {
        _controller = controller;
      },
    });
    const controller = _controller;

    const resp = new Response(rb);

    const [value, fn] = await Page();
    controller.enqueue(value)
    Readable.fromWeb(resp.body).pipe(res);

    await fn(controller);
    controller.close();
  });

  server.listen(3000, () => console.log('Started listening'));
}

main().catch(console.error);
