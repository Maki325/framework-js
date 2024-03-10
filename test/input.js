import { createServer } from 'http';

async function Coffee({ hotOrCold }) {
  const url = `https://api.sampleapis.com/coffee${hotOrCold}`;
  const res = await fetch(url);
  const coffees = await res.json();
  return <ul>{coffees.map(coffee => <li>{coffee.title}</li>)}</ul>
}

async function main() {
  const server = createServer(async (req, res) => {
    if (req.url === '/favicon.ico') {
      res.end();
      return;
    }

    res.write(
      <html>
        <body>
          These are the coffees:
          <ul id='some_fake-randomId' />
        </body>
      </html>
    );
    res.write(`
      <script id="script-random-id">
        document.getElementById("some_fake-randomId").outerHTML = "${<Coffee hotOrCold={req.url} />}";
        document.getElementById("script-random-id").remove();
      </script>
    `);
    res.end();
  });

  server.listen(3000, () => console.log('Started listening'));
}

main().catch(console.error);
