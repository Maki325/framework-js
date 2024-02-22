export function Hello({ name }: { name: string }): JSX.IntrinsicElements {
  return <div>Hello, {name}</div>;
}

<Hello name={'Marko'} />;
