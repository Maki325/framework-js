import { createServer } from 'http';
async function Coffee({ hotOrCold }) {
    const url = `https://api.sampleapis.com/coffee${hotOrCold}`;
    const res = await fetch(url);
    const coffees = await res.json();
    return `<ul>${(() => {const _FlDSgx5TXsTo=coffees.map((coffee)=>`<li>${(() => {const _6MgKgAOBnrPl=coffee.title; if(Array.isArray(_6MgKgAOBnrPl)) { return _6MgKgAOBnrPl.join(''); }else if(typeof _6MgKgAOBnrPl=== 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _6MgKgAOBnrPl;}})()}</li>`); if(Array.isArray(_FlDSgx5TXsTo)) { return _FlDSgx5TXsTo.join(''); }else if(typeof _FlDSgx5TXsTo=== 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _FlDSgx5TXsTo;}})()}</ul>`;
}
async function main() {
    const server = createServer(async (req, res)=>{
        if (req.url === '/favicon.ico') {
            res.end();
            return;
        }
        res.write(`<html>

        <body>

          These are the coffees:

          <ul id="some_fake-randomId"></ul>

        </body>

      </html>`);
        res.write(`
      <script id="script-random-id">
        document.getElementById("some_fake-randomId").outerHTML = "${await (async ()=>{
            const _GdswvDOUDZFm = Coffee({
                children: "",
                hotOrCold: req.url
            });
            if (_GdswvDOUDZFm instanceof Promise) return await _GdswvDOUDZFm;
            else return _GdswvDOUDZFm;
        })()}";
        document.getElementById("script-random-id").remove();
      </script>
    `);
        res.end();
    });
    server.listen(3000, ()=>console.log('Started listening'));
}
main().catch(console.error);
