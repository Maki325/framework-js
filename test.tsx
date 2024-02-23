function Wall({ children }: { children: string }) {
  return <div>A</div>;
}

<div>
  Hello <div>World</div>
  <Wall>
    <div>Marko</div>
  </Wall>
</div>;
