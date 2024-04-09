import { createServer } from 'http';
import Page from './component.js';
import { Readable } from 'stream';

global.___FRAMEWORK_JS_STRINGIFY___ = (item, toCreate) => {
  if (Array.isArray(item)) {
    if (
      item.length == 2 &&
      typeof item[0] === 'string' &&
      typeof item[1] === 'function'
    ) {
      toCreate.push(item[1]);
      return item[0];
    }
    return item.map(value => {
      if (
        Array.isArray(value) &&
        value.length == 2 &&
        typeof value[0] === 'string' &&
        typeof value[1] === 'function'
      ) {
        toCreate.push(value[1]);
        return value[0];
      } else {
        return value;
      }
    }).join('');
  } else if (typeof item === 'object') {
    throw new Exception('Objects are not valid as a JSX child!');
  } else {
    return item;
  }
}

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


    // const a = Page();
    // console.log('a 1:', a);
    // console.log('a 2:', await a);
    const [value, fn] = await Page();
    controller.enqueue(value)
    Readable.fromWeb(resp.body).pipe(res);
    // Readable.fromWeb(resp.body).pipe(process.stdout);
    // resp.body.pipeTo(res);
    // resp.body.pipe(res);
    await fn(controller);
    controller.close();

    // res.write(
    //   <html>
    //     <body>
    //       These are the coffees:
    //       <ul id='some_fake-randomId' />
    //     </body>
    //   </html>
    // );
    // res.write(`
    //   <script id="script-random-id">
    //     document.getElementById("some_fake-randomId").outerHTML = "${<Coffee hotOrCold={req.url} />}";
    //     document.getElementById("script-random-id").remove();
    //   </script>
    // `);
    // res.end();
  });

  server.listen(3000, () => console.log('Started listening'));
}

main().catch(console.error);
