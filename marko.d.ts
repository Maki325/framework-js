declare module global {
  const React: any;
}

declare namespace JSX {
  interface IntrinsicElements {
    div: {
      children?: any;
    };
  }
}
