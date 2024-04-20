import test from "node:test";
import assert from "node:assert";
import './impl.js';

test('stringify', async (t) => {
  await t.test('string', () => {
    const toCreate = [];
    assert.strictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___("string", toCreate),
      "string",
    );
    assert.deepStrictEqual(
      toCreate,
      [],
    );
  });

  await t.test('number', () => {
    const toCreate = [];
    assert.strictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(5, toCreate),
      5,
    );
    assert.deepStrictEqual(
      toCreate,
      [],
    );
  });

  await t.test('array', () => {
    const toCreate = [];
    assert.strictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(["hello", 1], toCreate),
      "hello1",
    );
    assert.deepStrictEqual(
      toCreate,
      [],
    );
  });

  await t.test('object', () => {
    const toCreate = [];
    assert.throws(
      () => global.___FRAMEWORK_JS_STRINGIFY___({ a: 5 }, toCreate),
      new Error('Objects are not valid as a JSX child!'),
    );
    assert.deepStrictEqual(
      toCreate,
      [],
    );
  });

  await t.test('JSX single sync html element', async () => {
    const toCreate = [];
    assert.deepStrictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(<div />, toCreate),
      "<div></div>",
    );

    assert.strictEqual(toCreate.length, 1);
    assert.strictEqual(typeof toCreate[0], 'function');
    
    const promise = toCreate[0]();
    assert.strictEqual(promise instanceof Promise, true);

    const result = await promise;
    assert.deepStrictEqual(result, []);
  });

  await t.test('JSX single sync html element with single text child', async () => {
    const toCreate = [];
    assert.deepStrictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(<h1>Hellooo</h1>, toCreate),
      "<h1>Hellooo</h1>",
    );

    assert.strictEqual(toCreate.length, 1);
    assert.strictEqual(typeof toCreate[0], 'function');
    
    const promise = toCreate[0]();
    assert.strictEqual(promise instanceof Promise, true);

    const result = await promise;
    assert.deepStrictEqual(result, []);
  });

  await t.test('JSX single sync html element with single sync html element child', async () => {
    const toCreate = [];
    assert.deepStrictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(<div><div /></div>, toCreate),
      "<div><div></div></div>",
    );

    assert.strictEqual(toCreate.length, 1);
    assert.strictEqual(typeof toCreate[0], 'function');
    
    const promise = toCreate[0]();
    assert.strictEqual(promise instanceof Promise, true);
    
    const result = await promise;
    assert.deepStrictEqual(result, []);
  });

  await t.test('JSX single sync custom element', async () => {
    const Component = () => <h1>Component</h1>;
    const toCreate = [];
    assert.deepStrictEqual(
      global.___FRAMEWORK_JS_STRINGIFY___(<Component />, toCreate),
      "<h1>Component</h1>",
    );

    assert.strictEqual(toCreate.length, 1);
    assert.strictEqual(typeof toCreate[0], 'function');
    
    const promise = toCreate[0]();
    assert.strictEqual(promise instanceof Promise, true);
    
    const result = await promise;
    assert.deepStrictEqual(Array.isArray(result), true);
    assert.deepStrictEqual(result.length, 1);

    assert.deepStrictEqual(result, [
      // PromiseSettledResult -> PromiseFulfilledResult
      {status: "fulfilled", value: []},
    ]);
  });

  await t.test('JSX single async custom element', async () => {
    const Component = async () => {
      await new Promise((resolve) => setTimeout(() => resolve(), 500));
      return <h1>Component</h1>;
    };

    const toCreate = [];
    const asyncDivRegex = /<div id="(_[A-Za-z0-9]{12})"><\/div>/;
    const asyncDiv = global.___FRAMEWORK_JS_STRINGIFY___(<Component />, toCreate);

    const matches = asyncDiv.match(asyncDivRegex);

    assert.strictEqual(matches.length, 2);
    assert.strictEqual(matches[0], asyncDiv);

    const asyncDivId = matches[1];

    assert.strictEqual(toCreate.length, 1);
    assert.strictEqual(typeof toCreate[0], 'function');

    let built = "";
    const builder = {
      enqueue: (part) => {
        built += part;
      }
    };
    
    const promise = toCreate[0](builder);
    assert.strictEqual(promise instanceof Promise, true);
    
    const result = await promise;
    assert.deepStrictEqual(Array.isArray(result), true);
    assert.deepStrictEqual(result.length, 1);

    assert.deepStrictEqual(result, [
      // PromiseSettledResult -> PromiseFulfilledResult
      {status: "fulfilled", value: []},
    ]);

    const scriptRegex = /<script id="(_[A-Za-z0-9]{12})">document\.getElementById\("(_[A-Za-z0-9]{12})"\)\.outerHTML = `<h1>Component<\/h1>`;document\.getElementById\("(_[A-Za-z0-9]{12})"\).remove\(\);<\/script>/;
    const scriptMatches = built.match(scriptRegex);

    assert.strictEqual(scriptMatches.length, 4);
    assert.strictEqual(scriptMatches[0], built);
    assert.strictEqual(scriptMatches[1], scriptMatches[3]);
    assert.strictEqual(scriptMatches[2], asyncDivId);
  });
});

test('parse style name', () => {
  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_NAME___("backgroundColor"),
    "background-color"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_NAME___("MozTransition"),
    "-moz-transition"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_NAME___("msTransition"),
    "-ms-transition"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_NAME___("--custom-css-var"),
    "--custom-css-var"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_NAME___("--cust'om-css-var"),
    "--cust&#x27;om-css-var"
  );
});

test('parse style value', () => {
  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_VALUE___("#123456", "backgroundColor"),
    "#123456"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_VALUE___(16, "fontSize"),
    "16px"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_VALUE___(1, "flex"),
    '1'
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_VALUE___(24, "--custom-css-var"),
    '24'
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_VALUE___("marko", "--custom-css-var"),
    "marko"
  );
});

test('parse style object', () => {
  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ backgroundColor: '#121212' }),
    "background-color: #121212"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ backgroundColor: '#121212', color: 'white' }),
    "background-color: #121212;color: white"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ fontSize: 16 }),
    "font-size: 16px"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ margin: 0 }),
    "margin: 0"
  );

  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ '--test': 'hello' }),
    "--test: hello"
  );

  const styles = { color: 'blue' };
  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ backgroundColor: '#121212', ...styles }),
    "background-color: #121212;color: blue"
  );

  const backgroundColor = 'red';
  assert.strictEqual(
    global.___FRAMEWORK_JS_STYLE_OBJECT___({ backgroundColor }),
    "background-color: red"
  );
});
