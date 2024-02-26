import { createServer } from 'http';
async function Coffee({ hotOrCold }) {
    const url = `https://api.sampleapis.com/coffee${hotOrCold}`;
    const res = await fetch(url);
    const coffees = await res.json();
    return `<ul>${(() => {const _TbIP5xv01I6z=coffees.map((coffee)=>`<li>${(() => {const _ywTmZoTbomCc=coffee.title; if(Array.isArray(_ywTmZoTbomCc)) { return _ywTmZoTbomCc.join(''); }else if(typeof _ywTmZoTbomCc=== 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _ywTmZoTbomCc;}})()}</li>`); if(Array.isArray(_TbIP5xv01I6z)) { return _TbIP5xv01I6z.join(''); }else if(typeof _TbIP5xv01I6z=== 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _TbIP5xv01I6z;}})()}</ul>`;
}
async function main() {
    const server = createServer(async (req, res)=>{
        if (req.url === '/favicon.ico') {
            res.end();
            return;
        }
        res.write(`<html>

        <body>

          ${await (async ()=>{
            const _c7HJay05ItCQ = Coffee({
                children: "",
                hotOrCold: req.url
            });
            if (_c7HJay05ItCQ instanceof Promise) return await _c7HJay05ItCQ;
            else return _c7HJay05ItCQ;
        })()}

        </body>

      </html>`);
        res.end();
    });
    server.listen(3000, ()=>console.log('Started listening'));
}
main().catch(console.error);
