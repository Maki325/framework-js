async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 300));
    return (()=>{
        const _6DagR6BQRUlJATr7 = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _6DagR6BQRUlJATr7 = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _6DagR6BQRUlJATr7)}</li>`,
                        (_NQP2HsEt3OzZ)=>{
                            const _UXjTmqUnApme = _6DagR6BQRUlJATr7.map((_KkgF6w7893EX)=>_KkgF6w7893EX(_NQP2HsEt3OzZ));
                            return Promise.allSettled(_UXjTmqUnApme);
                        }
                    ];
                })()), _6DagR6BQRUlJATr7)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _6DagR6BQRUlJATr7)}

    </ul>`,
            (_Qs2ut0QJU2yb)=>{
                const _SF2kAAVDTmZl = _6DagR6BQRUlJATr7.map((_rnucWoJgWrrt)=>_rnucWoJgWrrt(_Qs2ut0QJU2yb));
                return Promise.allSettled(_SF2kAAVDTmZl);
            }
        ];
    })();
}
const common = {
    Hello: function Hello({ name }) {
        const fontSize = '2em';
        const bStyle = {
            color: 'var(--test)',
            '--aa': '"hello"'
        };
        return (()=>{
            const _6DagR6BQRUlJATr7 = [];
            return [
                `<p style="font-size: ${global.___FRAMEWORK_JS_STYLE_VALUE___(fontSize, "fontSize")};color: red;margin: 0;padding: 0;--test: #1234AA">

        Hello 

        <b style="background-color: #fefefe;border-radius: 10px;padding: 0 5;${global.___FRAMEWORK_JS_STYLE_OBJECT___(bStyle)}">${global.___FRAMEWORK_JS_STRINGIFY___(name, _6DagR6BQRUlJATr7)}</b>!

      </p>`,
                (_I54ONVap5FHL)=>{
                    const _PidxVUe5x6DW = _6DagR6BQRUlJATr7.map((_BjJJnl9Rf4Hk)=>_BjJJnl9Rf4Hk(_I54ONVap5FHL));
                    return Promise.allSettled(_PidxVUe5x6DW);
                }
            ];
        })();
    }
};
function HTML({ title, children }) {
    return (()=>{
        const _6DagR6BQRUlJATr7 = [];
        return [
            `<html>

      <head>${global.___FRAMEWORK_JS_STRINGIFY___(title ? (()=>{
                const _6DagR6BQRUlJATr7 = [];
                return [
                    `<title>${global.___FRAMEWORK_JS_STRINGIFY___(title, _6DagR6BQRUlJATr7)}</title>`,
                    (_t6aVdRxWU9Ef)=>{
                        const _53Um8TkqeJ7a = _6DagR6BQRUlJATr7.map((_6fbVbcrpAy1f)=>_6fbVbcrpAy1f(_t6aVdRxWU9Ef));
                        return Promise.allSettled(_53Um8TkqeJ7a);
                    }
                ];
            })() : null, _6DagR6BQRUlJATr7)}</head>

      <body style="background-color: #121212;color: white">

        ${global.___FRAMEWORK_JS_STRINGIFY___(children, _6DagR6BQRUlJATr7)}

      </body>

    </html>`,
            (_XqQFykHloCdw)=>{
                const _WxE4e4up9tYz = _6DagR6BQRUlJATr7.map((_F0eSyHDCTSuZ)=>_F0eSyHDCTSuZ(_XqQFykHloCdw));
                return Promise.allSettled(_WxE4e4up9tYz);
            }
        ];
    })();
}
export default async function Page() {
    return (()=>{
        const _6DagR6BQRUlJATr7 = [];
        return [
            global.___FRAMEWORK_JS_STRINGIFY___(HTML({
                children: '\n\n      <div id="_ewoxENzZd9FL"></div>\n\n      <div id="_nd6roKlax2Vk"></div>\n\n    '
            }), _6DagR6BQRUlJATr7),
            (_ohjFTjErI1B7)=>{
                const _eiTqiRSxkrqI = _6DagR6BQRUlJATr7.map((_8bIf5UwuoFBX)=>_8bIf5UwuoFBX(_ohjFTjErI1B7));
                _eiTqiRSxkrqI.push((async ()=>{
                    const [_gDySm5DoOXMb, _vrMhr5RfqbvE] = await common.Hello({
                        children: "",
                        name: "Marko"
                    });
                    _ohjFTjErI1B7.enqueue(`<script id="_8GafSAoEKZYK">document.getElementById("_ewoxENzZd9FL").outerHTML = \`${_gDySm5DoOXMb.replace(/`/mg, "\\`")}\`;document.getElementById("_8GafSAoEKZYK").remove();</script>`);
                    _vrMhr5RfqbvE(_ohjFTjErI1B7);
                })());
                _eiTqiRSxkrqI.push((async ()=>{
                    const [_gDySm5DoOXMb, _vrMhr5RfqbvE] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _ohjFTjErI1B7.enqueue(`<script id="_7GRL2b2pD5X1">document.getElementById("_nd6roKlax2Vk").outerHTML = \`${_gDySm5DoOXMb.replace(/`/mg, "\\`")}\`;document.getElementById("_7GRL2b2pD5X1").remove();</script>`);
                    _vrMhr5RfqbvE(_ohjFTjErI1B7);
                })());
                return Promise.allSettled(_eiTqiRSxkrqI);
            }
        ];
    })();
}
