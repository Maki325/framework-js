import test from "node:test";
import assert from "node:assert";
import './impl.js';
test('stringify', async (t)=>{
    await t.test('string', ()=>{
        const toCreate = [];
        assert.strictEqual(global.___FRAMEWORK_JS_STRINGIFY___("string", toCreate), "string");
        assert.deepStrictEqual(toCreate, []);
    });
    await t.test('number', ()=>{
        const toCreate = [];
        assert.strictEqual(global.___FRAMEWORK_JS_STRINGIFY___(5, toCreate), 5);
        assert.deepStrictEqual(toCreate, []);
    });
    await t.test('array', ()=>{
        const toCreate = [];
        assert.strictEqual(global.___FRAMEWORK_JS_STRINGIFY___([
            "hello",
            1
        ], toCreate), "hello1");
        assert.deepStrictEqual(toCreate, []);
    });
    await t.test('object', ()=>{
        const toCreate = [];
        assert.throws(()=>global.___FRAMEWORK_JS_STRINGIFY___({
                a: 5
            }, toCreate), new Error('Objects are not valid as a JSX child!'));
        assert.deepStrictEqual(toCreate, []);
    });
    await t.test('JSX single sync html element', async ()=>{
        const toCreate = [];
        assert.deepStrictEqual(global.___FRAMEWORK_JS_STRINGIFY___((()=>{
            const _lV6X7BJFimIUSBwF = [];
            return [
                `<div></div>`,
                (_SrPHLeBEvOxv)=>{
                    const _zkMF8653rTJt = _lV6X7BJFimIUSBwF.map((_Bf6FZuMdglWO)=>_Bf6FZuMdglWO(_SrPHLeBEvOxv));
                    return Promise.allSettled(_zkMF8653rTJt);
                }
            ];
        })(), toCreate), "<div></div>");
        assert.strictEqual(toCreate.length, 1);
        assert.strictEqual(typeof toCreate[0], 'function');
        const promise = toCreate[0]();
        assert.strictEqual(promise instanceof Promise, true);
        const result = await promise;
        assert.deepStrictEqual(result, []);
    });
    await t.test('JSX single sync html element with single text child', async ()=>{
        const toCreate = [];
        assert.deepStrictEqual(global.___FRAMEWORK_JS_STRINGIFY___((()=>{
            const _lV6X7BJFimIUSBwF = [];
            return [
                `<h1>Hellooo</h1>`,
                (_9LCLbR84POYV)=>{
                    const _cuLE2JHDQF3X = _lV6X7BJFimIUSBwF.map((_5FrDzFaYjrbK)=>_5FrDzFaYjrbK(_9LCLbR84POYV));
                    return Promise.allSettled(_cuLE2JHDQF3X);
                }
            ];
        })(), toCreate), "<h1>Hellooo</h1>");
        assert.strictEqual(toCreate.length, 1);
        assert.strictEqual(typeof toCreate[0], 'function');
        const promise = toCreate[0]();
        assert.strictEqual(promise instanceof Promise, true);
        const result = await promise;
        assert.deepStrictEqual(result, []);
    });
    await t.test('JSX single sync html element with single sync html element child', async ()=>{
        const toCreate = [];
        assert.deepStrictEqual(global.___FRAMEWORK_JS_STRINGIFY___((()=>{
            const _lV6X7BJFimIUSBwF = [];
            return [
                `<div><div></div></div>`,
                (_roqjuq8lE5ss)=>{
                    const _Kn3jSwuTZJpK = _lV6X7BJFimIUSBwF.map((_ME7RE3TLBM4Z)=>_ME7RE3TLBM4Z(_roqjuq8lE5ss));
                    return Promise.allSettled(_Kn3jSwuTZJpK);
                }
            ];
        })(), toCreate), "<div><div></div></div>");
        assert.strictEqual(toCreate.length, 1);
        assert.strictEqual(typeof toCreate[0], 'function');
        const promise = toCreate[0]();
        assert.strictEqual(promise instanceof Promise, true);
        const result = await promise;
        assert.deepStrictEqual(result, []);
    });
    await t.test('JSX single sync custom element', async ()=>{
        const Component = ()=>(()=>{
                const _lV6X7BJFimIUSBwF = [];
                return [
                    `<h1>Component</h1>`,
                    (_7HyqSfxaHdSA)=>{
                        const _Z71S6zNxkD0o = _lV6X7BJFimIUSBwF.map((_oLlV7HlP9l9Y)=>_oLlV7HlP9l9Y(_7HyqSfxaHdSA));
                        return Promise.allSettled(_Z71S6zNxkD0o);
                    }
                ];
            })();
        const toCreate = [];
        assert.deepStrictEqual(global.___FRAMEWORK_JS_STRINGIFY___((()=>{
            const _lV6X7BJFimIUSBwF = [];
            return [
                global.___FRAMEWORK_JS_STRINGIFY___(Component({
                    children: ""
                }), _lV6X7BJFimIUSBwF),
                (_GzOiGspiOcIO)=>{
                    const _xp3vEWEzy4qI = _lV6X7BJFimIUSBwF.map((_kcZHSIffF167)=>_kcZHSIffF167(_GzOiGspiOcIO));
                    return Promise.allSettled(_xp3vEWEzy4qI);
                }
            ];
        })(), toCreate), "<h1>Component</h1>");
        assert.strictEqual(toCreate.length, 1);
        assert.strictEqual(typeof toCreate[0], 'function');
        const promise = toCreate[0]();
        assert.strictEqual(promise instanceof Promise, true);
        const result = await promise;
        assert.deepStrictEqual(Array.isArray(result), true);
        assert.deepStrictEqual(result.length, 1);
        assert.deepStrictEqual(result, [
            {
                status: "fulfilled",
                value: []
            }
        ]);
    });
    await t.test('JSX single async custom element', async ()=>{
        const Component = async ()=>{
            await new Promise((resolve)=>setTimeout(()=>resolve(), 500));
            return (()=>{
                const _lV6X7BJFimIUSBwF = [];
                return [
                    `<h1>Component</h1>`,
                    (_oxtcE7wg9l1J)=>{
                        const _BpSzPaPDc24X = _lV6X7BJFimIUSBwF.map((_UBzCjZdRCrbu)=>_UBzCjZdRCrbu(_oxtcE7wg9l1J));
                        return Promise.allSettled(_BpSzPaPDc24X);
                    }
                ];
            })();
        };
        const toCreate = [];
        const asyncDivRegex = /<div id="(_[A-Za-z0-9]{12})"><\/div>/;
        const asyncDiv = global.___FRAMEWORK_JS_STRINGIFY___((()=>{
            const _lV6X7BJFimIUSBwF = [];
            return [
                '<div id="_Ye1sHZhrfkLB"></div>',
                (_Fic7fV6lj0lE)=>{
                    const _4FFLAKozUEOl = _lV6X7BJFimIUSBwF.map((_wjy0W0MMEBQo)=>_wjy0W0MMEBQo(_Fic7fV6lj0lE));
                    _4FFLAKozUEOl.push((async ()=>{
                        const [_nzkz4xKdxign, _fHkCJoqvzEsU] = await Component({
                            children: ""
                        });
                        _Fic7fV6lj0lE.enqueue(`<script id="_Ceud2UNLCdcs">document.getElementById("_Ye1sHZhrfkLB").outerHTML = \`${_nzkz4xKdxign.replace(/`/mg, "\\`")}\`;document.getElementById("_Ceud2UNLCdcs").remove();</script>`);
                        return _fHkCJoqvzEsU(_Fic7fV6lj0lE);
                    })());
                    return Promise.allSettled(_4FFLAKozUEOl);
                }
            ];
        })(), toCreate);
        const matches = asyncDiv.match(asyncDivRegex);
        assert.strictEqual(matches.length, 2);
        assert.strictEqual(matches[0], asyncDiv);
        const asyncDivId = matches[1];
        assert.strictEqual(toCreate.length, 1);
        assert.strictEqual(typeof toCreate[0], 'function');
        let built = "";
        const builder = {
            enqueue: (part)=>{
                built += part;
            }
        };
        const promise = toCreate[0](builder);
        assert.strictEqual(promise instanceof Promise, true);
        const result = await promise;
        assert.deepStrictEqual(Array.isArray(result), true);
        assert.deepStrictEqual(result.length, 1);
        assert.deepStrictEqual(result, [
            {
                status: "fulfilled",
                value: []
            }
        ]);
        const scriptRegex = /<script id="(_[A-Za-z0-9]{12})">document\.getElementById\("(_[A-Za-z0-9]{12})"\)\.outerHTML = `<h1>Component<\/h1>`;document\.getElementById\("(_[A-Za-z0-9]{12})"\).remove\(\);<\/script>/;
        const scriptMatches = built.match(scriptRegex);
        assert.strictEqual(scriptMatches.length, 4);
        assert.strictEqual(scriptMatches[0], built);
        assert.strictEqual(scriptMatches[1], scriptMatches[3]);
        assert.strictEqual(scriptMatches[2], asyncDivId);
    });
});
test('parse style name', ()=>{
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_NAME___("backgroundColor"), "background-color");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_NAME___("MozTransition"), "-moz-transition");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_NAME___("msTransition"), "-ms-transition");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_NAME___("--custom-css-var"), "--custom-css-var");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_NAME___("--cust'om-css-var"), "--cust&#x27;om-css-var");
});
test('parse style value', ()=>{
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_VALUE___("#123456", "backgroundColor"), "#123456");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_VALUE___(16, "fontSize"), "16px");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_VALUE___(1, "flex"), '1');
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_VALUE___(24, "--custom-css-var"), '24');
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_VALUE___("marko", "--custom-css-var"), "marko");
});
test('parse style object', ()=>{
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        backgroundColor: '#121212'
    }), "background-color: #121212");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        backgroundColor: '#121212',
        color: 'white'
    }), "background-color: #121212;color: white");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        fontSize: 16
    }), "font-size: 16px");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        margin: 0
    }), "margin: 0");
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        '--test': 'hello'
    }), "--test: hello");
    const styles = {
        color: 'blue'
    };
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        backgroundColor: '#121212',
        ...styles
    }), "background-color: #121212;color: blue");
    const backgroundColor = 'red';
    assert.strictEqual(global.___FRAMEWORK_JS_STYLE_OBJECT___({
        backgroundColor
    }), "background-color: red");
});
