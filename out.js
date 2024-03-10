function Wall(_HvqagumPT8Dvs1GK, _W3T89kr4w7tzxsC8, { children, ...props }) {
    return `<div ${Object.entries(props).map(([key, value]) => `${key}="${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/"/mg, '\\"') : 'true'}"`).join(' ')}>A</div>`;
}
let q = function z() {
    function b(_HvqagumPT8Dvs1GK, _W3T89kr4w7tzxsC8) {
        return await (async ()=>{
            const _xgx4JVnpzagk = A({
                children: ""
            });
            if (_xgx4JVnpzagk instanceof Promise) return await _xgx4JVnpzagk;
            else return _xgx4JVnpzagk;
        })();
    }
    return b();
};
const props = {
    hello: 'world'
};
`<div>

  Hello${(() => {const _qlnc81SXBBzM=' '; if(Array.isArray(_qlnc81SXBBzM)) { return _qlnc81SXBBzM.join(''); }else if(typeof _qlnc81SXBBzM=== 'object') { throw new Exception('Objects are not valid as a React child!') } else { return _qlnc81SXBBzM;}})()}

  <div ${Object.entries(props).map(([key, value]) => `${key}="${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/"/mg, '\\"') : 'true'}"`).join(' ')} test="[
    'Hellooo \"Maki325\"'
]">

    World

  </div>

  ${await (async ()=>{
    const _47EwBTZvPGEL = Wall({
        children: "\n\n    <div>Marko</div>\n\n  ",
        maki325: [
            'is',
            'the',
            'best',
            '!'
        ],
        hello: "world",
        number: 5,
        truthy: true,
        bool: false,
        nully: null,
        re: /hello/m,
        jsxIntrinsic: `<h1>Maki325 is the best</h1>`,
        jsxCustomElement: await (async ()=>{
            const _WARtURHdDnhD = Wall({
                children: "Maki325"
            });
            if (_WARtURHdDnhD instanceof Promise) return await _WARtURHdDnhD;
            else return _WARtURHdDnhD;
        })()
    });
    if (_47EwBTZvPGEL instanceof Promise) return await _47EwBTZvPGEL;
    else return _47EwBTZvPGEL;
})()}

</div>`;
