async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 1000));
    return (()=>{
        const _F6DkrlpkUfdhwqPR = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _F6DkrlpkUfdhwqPR = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _F6DkrlpkUfdhwqPR)}</li>`,
                        (_Uc6J8sGmLwZi)=>{
                            const _8WxSfPWH78Rx = _F6DkrlpkUfdhwqPR.map((_ODU8GuBwZd8e)=>_ODU8GuBwZd8e(_Uc6J8sGmLwZi));
                            return Promise.allSettled(_8WxSfPWH78Rx);
                        }
                    ];
                })()), _F6DkrlpkUfdhwqPR)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _F6DkrlpkUfdhwqPR)}

    </ul>`,
            (_ZA1l0tyfjohE)=>{
                const _ovs7fJzwUzXw = _F6DkrlpkUfdhwqPR.map((_20q4EVA2W0BV)=>_20q4EVA2W0BV(_ZA1l0tyfjohE));
                return Promise.allSettled(_ovs7fJzwUzXw);
            }
        ];
    })();
}
export default async function Page() {
    return (()=>{
        const _F6DkrlpkUfdhwqPR = [];
        return [
            `<div>

      <h1>Helloooo</h1>

      <div id="_mQGBKXICTw64"></div>

    </div>`,
            (_wCS63IHxrlyV)=>{
                const _KW5ZdTDWxnW2 = _F6DkrlpkUfdhwqPR.map((_IPktqL6usypu)=>_IPktqL6usypu(_wCS63IHxrlyV));
                _KW5ZdTDWxnW2.push((async ()=>{
                    const [_N6EbDI45qdZo, _zeAfDZ63DjnK] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _wCS63IHxrlyV.enqueue(`<script id="_CD87TIkCWedV">document.getElementById("_mQGBKXICTw64").outerHTML = \`${_N6EbDI45qdZo.replace(/`/mg, "\\`")}\`;document.getElementById("_CD87TIkCWedV").remove();</script>`);
                    _zeAfDZ63DjnK(_wCS63IHxrlyV);
                })());
                return Promise.allSettled(_KW5ZdTDWxnW2);
            }
        ];
    })();
}
