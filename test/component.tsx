import { PropsWithChildren } from 'react';

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

const common = {
  Hello: function Hello({ name }: { name: string }) {
    const fontSize = '2em';
    const bStyle = { color: 'var(--test)', '--aa': '"hello"' };
    return (
      <p
        style={{
          fontSize,
          color: 'red',
          margin: 0,
          padding: 0,
          '--test': '#1234AA',
        }}>
        Hello{' '}
        <b style={{ backgroundColor: '#fefefe', borderRadius: 10, padding: '0 5', ...bStyle }}>{name}</b>!
      </p>
    );
  },
};

type HTMLProps = {
  title?: string;
};

function HTML({ title, children }: PropsWithChildren<HTMLProps>) {
  return (
    <html>
      <head>{title ? <title>{title}</title> : null}</head>
      <body
        style={{
          backgroundColor: '#121212',
          color: 'white',
        }}>
        {children}
      </body>
    </html>
  );
}

export default async function Page() {
  return (
    <HTML>
      <common.Hello name="Marko" />
      <Coffee hotOrIced="iced">
        <h1>STUFF`</h1>
      </Coffee>
    </HTML>
  );
}
