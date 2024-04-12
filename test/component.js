async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 300));
    return (()=>{
        const _gNYEFyfpOte4Zn2r = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _gNYEFyfpOte4Zn2r = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _gNYEFyfpOte4Zn2r)}</li>`,
                        (_k68yWAKK2odz)=>{
                            const _wZ0XT2XuhjK3 = _gNYEFyfpOte4Zn2r.map((_Fe5efERbDO7u)=>_Fe5efERbDO7u(_k68yWAKK2odz));
                            return Promise.allSettled(_wZ0XT2XuhjK3);
                        }
                    ];
                })()), _gNYEFyfpOte4Zn2r)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _gNYEFyfpOte4Zn2r)}

    </ul>`,
            (_KDPQibfCgabu)=>{
                const _A7zZejiIJ5vI = _gNYEFyfpOte4Zn2r.map((_wDWsBj23sUNF)=>_wDWsBj23sUNF(_KDPQibfCgabu));
                return Promise.allSettled(_A7zZejiIJ5vI);
            }
        ];
    })();
}
const common = {
    Hello: function Hello({ name }) {
        return (()=>{
            const _gNYEFyfpOte4Zn2r = [];
            return [
                `<p style="font-size: 2em">

        Hello <b>${global.___FRAMEWORK_JS_STRINGIFY___(name, _gNYEFyfpOte4Zn2r)}</b>!

      </p>`,
                (_SqWfj92EC12J)=>{
                    const _t9NI02MtQxTh = _gNYEFyfpOte4Zn2r.map((_Vh9XrI6AZWjI)=>_Vh9XrI6AZWjI(_SqWfj92EC12J));
                    return Promise.allSettled(_t9NI02MtQxTh);
                }
            ];
        })();
    }
};
export default async function Page() {
    return (()=>{
        const _gNYEFyfpOte4Zn2r = [];
        return [
            `<div>

      <div id="_PeW2UIzIeocP"></div>

      <div id="_Cj5vxbwILu0W"></div>

    </div>`,
            (_Rl9lrk4lb7Vg)=>{
                const _s9fAtJtFxlV8 = _gNYEFyfpOte4Zn2r.map((_OGdRzFJKURE0)=>_OGdRzFJKURE0(_Rl9lrk4lb7Vg));
                _s9fAtJtFxlV8.push((async ()=>{
                    const [_5pwBYZzIGwsV, _SmX0VSNldthv] = await common.Hello({
                        children: "",
                        name: "Marko"
                    });
                    _Rl9lrk4lb7Vg.enqueue(`<script id="_dJrYGFESk4J1">document.getElementById("_PeW2UIzIeocP").outerHTML = \`${_5pwBYZzIGwsV.replace(/`/mg, "\\`")}\`;document.getElementById("_dJrYGFESk4J1").remove();</script>`);
                    _SmX0VSNldthv(_Rl9lrk4lb7Vg);
                })());
                _s9fAtJtFxlV8.push((async ()=>{
                    const [_5pwBYZzIGwsV, _SmX0VSNldthv] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _Rl9lrk4lb7Vg.enqueue(`<script id="_51WMMJy7KnmZ">document.getElementById("_Cj5vxbwILu0W").outerHTML = \`${_5pwBYZzIGwsV.replace(/`/mg, "\\`")}\`;document.getElementById("_51WMMJy7KnmZ").remove();</script>`);
                    _SmX0VSNldthv(_Rl9lrk4lb7Vg);
                })());
                return Promise.allSettled(_s9fAtJtFxlV8);
            }
        ];
    })();
}
