async function Coffee({ hotOrIced, children }) {
    const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
    const res = await fetch(url);
    const coffees = await res.json();
    await new Promise((resolve)=>setTimeout(()=>resolve(), 300));
    return (()=>{
        const _k8YIKRf1yQ0pXztX = [];
        return [
            `<ul>

      ${global.___FRAMEWORK_JS_STRINGIFY___(coffees.map((coffee)=>(()=>{
                    const _k8YIKRf1yQ0pXztX = [];
                    return [
                        `<li>${global.___FRAMEWORK_JS_STRINGIFY___(coffee.title, _k8YIKRf1yQ0pXztX)}</li>`,
                        (_7tKMUIkV9tj1)=>{
                            const _6IQk0MquA7be = _k8YIKRf1yQ0pXztX.map((_l4JD9bGyf5Bg)=>_l4JD9bGyf5Bg(_7tKMUIkV9tj1));
                            return Promise.allSettled(_6IQk0MquA7be);
                        }
                    ];
                })()), _k8YIKRf1yQ0pXztX)}

      ${global.___FRAMEWORK_JS_STRINGIFY___(children, _k8YIKRf1yQ0pXztX)}

    </ul>`,
            (_IQZiGOtWFR0u)=>{
                const _hX0yYkmfSu9r = _k8YIKRf1yQ0pXztX.map((_HmihAMog9Aer)=>_HmihAMog9Aer(_IQZiGOtWFR0u));
                return Promise.allSettled(_hX0yYkmfSu9r);
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
            const _k8YIKRf1yQ0pXztX = [];
            return [
                `<p style="font-size: ${global.___FRAMEWORK_JS_STYLE_VALUE___(fontSize, "fontSize")};color: red;margin: 0;padding: 0;--test: #1234AA">

        Hello <b style="background-color: black;${global.___FRAMEWORK_JS_STYLE_OBJECT___(bStyle)}">${global.___FRAMEWORK_JS_STRINGIFY___(name, _k8YIKRf1yQ0pXztX)}</b>!

      </p>`,
                (_ckD7ctWJpMCo)=>{
                    const _CiwI9wMicLWR = _k8YIKRf1yQ0pXztX.map((_F1Sxz1PuhhU7)=>_F1Sxz1PuhhU7(_ckD7ctWJpMCo));
                    return Promise.allSettled(_CiwI9wMicLWR);
                }
            ];
        })();
    }
};
export default async function Page() {
    return (()=>{
        const _k8YIKRf1yQ0pXztX = [];
        return [
            `<div>

      <div id="_OST2hG7gejCx"></div>

      <div id="_SxDuHqf7YfMp"></div>

    </div>`,
            (_laL9lC3qtsQi)=>{
                const _ivx41kjOiwHt = _k8YIKRf1yQ0pXztX.map((_L3TB0y3I5f9s)=>_L3TB0y3I5f9s(_laL9lC3qtsQi));
                _ivx41kjOiwHt.push((async ()=>{
                    const [_Ewfdyvis5h7S, _4EmMsEGzstTi] = await common.Hello({
                        children: "",
                        name: "Marko"
                    });
                    _laL9lC3qtsQi.enqueue(`<script id="_puWM0MFOWOvx">document.getElementById("_OST2hG7gejCx").outerHTML = \`${_Ewfdyvis5h7S.replace(/`/mg, "\\`")}\`;document.getElementById("_puWM0MFOWOvx").remove();</script>`);
                    _4EmMsEGzstTi(_laL9lC3qtsQi);
                })());
                _ivx41kjOiwHt.push((async ()=>{
                    const [_Ewfdyvis5h7S, _4EmMsEGzstTi] = await Coffee({
                        children: "\n\n        <h1>STUFF`</h1>\n\n      ",
                        hotOrIced: "iced"
                    });
                    _laL9lC3qtsQi.enqueue(`<script id="_WmiKTM8emou5">document.getElementById("_SxDuHqf7YfMp").outerHTML = \`${_Ewfdyvis5h7S.replace(/`/mg, "\\`")}\`;document.getElementById("_WmiKTM8emou5").remove();</script>`);
                    _4EmMsEGzstTi(_laL9lC3qtsQi);
                })());
                return Promise.allSettled(_ivx41kjOiwHt);
            }
        ];
    })();
}
