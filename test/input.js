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
          <Coffee hotOrCold={req.url} />
        </body>
      </html>
    );
    res.end();
  });

  server.listen(3000, () => console.log('Started listening'));
}

main().catch(console.error);
