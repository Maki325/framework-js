async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 300));
    return (()=>{
        const _SG6IBxDUvS2VAtp8 = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _SG6IBxDUvS2VAtp8 = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _SG6IBxDUvS2VAtp8)}</li>`,
                        (_ZrUsbfP7Cd5n)=>{
                            const _k6AqoD0mngpO = _SG6IBxDUvS2VAtp8.map((_JXz7rPP3EJ1m)=>_JXz7rPP3EJ1m(_ZrUsbfP7Cd5n));
                            return Promise.allSettled(_k6AqoD0mngpO);
                        }
                    ];
                })()), _SG6IBxDUvS2VAtp8)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _SG6IBxDUvS2VAtp8)}

    </ul>`,
            (_H8gVgvz4hKTy)=>{
                const _oHVj7goKooNL = _SG6IBxDUvS2VAtp8.map((_WgRkdRgwTYxz)=>_WgRkdRgwTYxz(_H8gVgvz4hKTy));
                return Promise.allSettled(_oHVj7goKooNL);
            }
        ];
    })();
}
const common = {
    Hello: function Hello({ name }) {
        const fontSize = '2em';
        return (()=>{
            const _SG6IBxDUvS2VAtp8 = [];
            return [
                `<p style="font-size: ${global.___FRAMEWORK_JS_STYLE_VALUE___(fontSize)};color: red;margin: 0;padding: 0">

        Hello <b>${global.___FRAMEWORK_JS_STRINGIFY___(name, _SG6IBxDUvS2VAtp8)}</b>!

      </p>`,
                (_uyNUUoZ5oHgG)=>{
                    const _GquDnjxy5Gk6 = _SG6IBxDUvS2VAtp8.map((_sNunwx42q8nW)=>_sNunwx42q8nW(_uyNUUoZ5oHgG));
                    return Promise.allSettled(_GquDnjxy5Gk6);
                }
            ];
        })();
    }
};
export default async function Page() {
    return (()=>{
        const _SG6IBxDUvS2VAtp8 = [];
        return [
            `<div>

      <div id="_qcEMxGGvJ2Di"></div>

      <div id="_49oRclnWxFF9"></div>

    </div>`,
            (_rymKHKiR5ofq)=>{
                const _zOi2Hvno2nLb = _SG6IBxDUvS2VAtp8.map((_sclLtbkEtizh)=>_sclLtbkEtizh(_rymKHKiR5ofq));
                _zOi2Hvno2nLb.push((async ()=>{
                    const [_hN2auS28drjs, _qJyS1o6mK0Hj] = await common.Hello({
                        children: "",
                        name: "Marko"
                    });
                    _rymKHKiR5ofq.enqueue(`<script id="_ApRmNc5q7gaF">document.getElementById("_qcEMxGGvJ2Di").outerHTML = \`${_hN2auS28drjs.replace(/`/mg, "\\`")}\`;document.getElementById("_ApRmNc5q7gaF").remove();</script>`);
                    _qJyS1o6mK0Hj(_rymKHKiR5ofq);
                })());
                _zOi2Hvno2nLb.push((async ()=>{
                    const [_hN2auS28drjs, _qJyS1o6mK0Hj] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _rymKHKiR5ofq.enqueue(`<script id="_UPvoIF7FEAor">document.getElementById("_49oRclnWxFF9").outerHTML = \`${_hN2auS28drjs.replace(/`/mg, "\\`")}\`;document.getElementById("_UPvoIF7FEAor").remove();</script>`);
                    _qJyS1o6mK0Hj(_rymKHKiR5ofq);
                })());
                return Promise.allSettled(_zOi2Hvno2nLb);
            }
        ];
    })();
}
