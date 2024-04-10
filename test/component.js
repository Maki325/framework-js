async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 300));
    return (()=>{
        const _UasdlViVSAql6cPv = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _UasdlViVSAql6cPv = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _UasdlViVSAql6cPv)}</li>`,
                        (_T5n7PWKio7x4)=>{
                            const _SpVqQBfoGE9r = _UasdlViVSAql6cPv.map((_R1z8I6F7NvWJ)=>_R1z8I6F7NvWJ(_T5n7PWKio7x4));
                            return Promise.allSettled(_SpVqQBfoGE9r);
                        }
                    ];
                })()), _UasdlViVSAql6cPv)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _UasdlViVSAql6cPv)}

    </ul>`,
            (_b4bxPQn83loE)=>{
                const _eEdslAsyjt71 = _UasdlViVSAql6cPv.map((_jSEMzMEB3o25)=>_jSEMzMEB3o25(_b4bxPQn83loE));
                return Promise.allSettled(_eEdslAsyjt71);
            }
        ];
    })();
}
function Hello({ name }) {
    return (()=>{
        const _UasdlViVSAql6cPv = [];
        return [
            `<p style="font-size: 2em">

      Hello <b>${global.___FRAMEWORK_JS_STRINGIFY___(name, _UasdlViVSAql6cPv)}</b>!

    </p>`,
            (_UoWT75YPMjgQ)=>{
                const _6fHoShYBh6WT = _UasdlViVSAql6cPv.map((_hjxXbYMUsORr)=>_hjxXbYMUsORr(_UoWT75YPMjgQ));
                return Promise.allSettled(_6fHoShYBh6WT);
            }
        ];
    })();
}
export default async function Page() {
    return (()=>{
        const _UasdlViVSAql6cPv = [];
        return [
            `<div>

      ${global.___FRAMEWORK_JS_STRINGIFY___(Hello({
                children: "",
                name: "Marko"
            }), _UasdlViVSAql6cPv)}

      <div id="_rJ4QeAMvCe2O"></div>

    </div>`,
            (_Br2KlrmGjjdU)=>{
                const _wFkAOjXKUzvw = _UasdlViVSAql6cPv.map((_9Ic8WQV5E1eZ)=>_9Ic8WQV5E1eZ(_Br2KlrmGjjdU));
                _wFkAOjXKUzvw.push((async ()=>{
                    const [_CpOfILNQ3QMh, _rGeKDNzyrxBP] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _Br2KlrmGjjdU.enqueue(`<script id="_nDPXBDVgSAEr">document.getElementById("_rJ4QeAMvCe2O").outerHTML = \`${_CpOfILNQ3QMh.replace(/`/mg, "\\`")}\`;document.getElementById("_nDPXBDVgSAEr").remove();</script>`);
                    _rGeKDNzyrxBP(_Br2KlrmGjjdU);
                })());
                return Promise.allSettled(_wFkAOjXKUzvw);
            }
        ];
    })();
}
