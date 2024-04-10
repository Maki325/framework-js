async function Coffee({
  hotOrIced,
  children,
}: {
  hotOrIced: 'hot' | 'iced';
  children: JSX.Element;
}) {
  const url = `https://api.sampleapis.com/coffee/${hotOrIced}`;
  const res = await fetch(url);
  const coffees = await res.json();
  await new Promise<void>((resolve) => setTimeout(() => resolve(), 300));
  return (
    <ul>
      {coffees.map((coffee: any) => (
        <li>{coffee.title}</li>
      ))}
      {children}
    </ul>
  );
}

function Hello({ name }: { name: string }) {
  return (
    <p style="font-size: 2em">
      Hello <b>{name}</b>!
    </p>
  );
}

export default async function Page() {
  return (
    <div>
      <Hello name="Marko" />
      <Coffee hotOrIced="iced">
        <h1>STUFF`</h1>
      </Coffee>
    </div>
  );
}
