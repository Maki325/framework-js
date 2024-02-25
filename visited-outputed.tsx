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
    children: "\n\n    <div>Marko</div>\n\n  "
})}

</div>`;
