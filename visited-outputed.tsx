function Wall({ children, ...props }: {
    children: string;
}) {
    return `<div  ${Object.entries(props).map(([key, value]) => `${key}="${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/"/mg, '\\"') : 'true'}"`).join(' ')}>A</div>`;
}
const props: {
    [key: string]: unknown;
} = {
    hello: 'world'
};
`<div >

  Hello 

  <div  ${Object.entries(props).map(([key, value]) => `${key}="${value ? (typeof value === 'string' ? value : (value instanceof RegExp ? value.toString() : JSON.stringify(value))).replace(/"/mg, '\\"') : 'true'}"`).join(' ')} test="[
    'Hellooo \"Maki325\"'
]">

    World

  </div>

  ${Wall({
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
    jsxIntrinsic: "<h1>Maki325 is the best</h1>",
    jsxCustomElement: Wall({
        children: "Maki325"
    })
})}

</div>`;
