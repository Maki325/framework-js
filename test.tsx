function Wall({ children, ...props }: { children: string }) {
  return <div {...props}>A</div>;
}

let q = function z() {
  function b() {
    return <A />;
  }
  return b();
};

const props: { [key: string]: unknown } = { hello: 'world' };

<div>
  Hello{' '}
  <div {...props} test={['Hellooo "Maki325"']}>
    World
  </div>
  <Wall
    maki325={['is', 'the', 'best', '!']}
    hello="world"
    number={5}
    truthy
    bool={false}
    nully={null}
    re={/hello/m}
    jsxIntrinsic={<h1>Maki325 is the best</h1>}
    jsxCustomElement={<Wall>Maki325</Wall>}>
    <div>Marko</div>
  </Wall>
</div>;
