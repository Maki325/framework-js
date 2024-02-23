function Wall({ children }: {
    children: string;
}) {
    return "<div>A</div>";
}
`<div>${`

  Hello <div>World</div>

  ${Wall({
    children: "\n\n    <div>Marko</div>\n\n  "
})}

`}</div>`;
