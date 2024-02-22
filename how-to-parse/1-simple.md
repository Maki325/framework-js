# How to parse

## Step 1: Input

```jsx
function Hello({ name }: { name: string }): JSX.IntrinsicElements {
  return <div>Hello, {name}</div>;
}

export function Page() {
  return (
    <Hello name={'Marko'} />;
  )
}
```

## Step 2: Convert functions so that they return pure HTML strings

```js
function Hello({ name }) {
  return `<div>Hello, ${name}</div>`;
}

export function Page() {
  return Hello({name: 'Marko'});
}
```
