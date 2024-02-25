function Wall({ children, ...props }: { children: string }) {
  return <div {...props}>A</div>;
}

const props: { [key: string]: unknown } = { hello: 'world' };

<div>
  Hello{' '}
  <div {...props} test={['Hellooo "Maki325"']}>
    World
  </div>
  <Wall>
    <div>Marko</div>
  </Wall>
</div>;
