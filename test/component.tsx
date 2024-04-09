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
  await new Promise<void>((resolve) => setTimeout(() => resolve(), 1000));
  return (
    <ul>
      {coffees.map((coffee: any) => (
        <li>{coffee.title}</li>
      ))}
      {children}
    </ul>
  );
}

export default async function Page() {
  return (
    <div>
      <h1>Helloooo</h1>
      <Coffee hotOrIced="iced">
        <h1>STUFF`</h1>
      </Coffee>
    </div>
  );
}
