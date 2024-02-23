# How to parse

## Step 1: Input

```jsx
function Hello({ name }: { name: string }): JSX.IntrinsicElements {
  return <div>Hello, {name}</div>;
}

function Navigation(): JSX.IntrinsicElements {
  return (
    <nav>
      <a href="#">Home</a>
      <a href="/work">Work</a>
    </nav>
  );
}

function Layout() {
  return (
    <>
      <Navigation />;
      <Hello name={'Marko'} />;
    </>
  )
}
```

## Step 2: Convert functions so that they return pure HTML strings

```js
function Hello({ name }) {
  return `<div>Hello, ${name}</div>`;
}

function Navigation() {
  return `<nav><a href="#">Home</a><a href="/work">Work</a></nav>`;
}

function Layout() {
  return `<main>${Navigation()}${Hello({name: 'Marko'})}</main>`;
}
```
