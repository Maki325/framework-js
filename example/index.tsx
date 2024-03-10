async function Coffee({ kind }: { kind: 'hot' | 'iced' }) {
  const url = `https://api.sampleapis.com/coffee/${kind}`;
  const res = await fetch(url);
  const coffees = await res.json();
  let a = (
    <ul>
      {coffees.map((coffee: { title: string }) => (
        <li>{coffee.title}</li>
      ))}
    </ul>
  );
  return a;
}

// Hey, we don't allow JSX outside of functions, thx
let a = <Coffee kind="hot" />;

export default Coffee;
