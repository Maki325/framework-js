# How to parse

## Step 1: Input

```jsx
function Navigation(): JSX.IntrinsicElements {
  return (
    <nav>
      <a href="#">Home</a>
      <a href="/work">Work</a>
    </nav>
  );
}

function Layout({children}) {
  return (
    <>
      <Navigation />;
      {children}
    </>
  )
}
```

## Step 2: Convert functions so that they return pure HTML strings

Prop `children` will ALWAYS be converted into a string before sending it as a "prop".

```js
function Navigation() {
  return `<nav><a href="#">Home</a><a href="/work">Work</a></nav>`;
}

function Layout({children}) {
  return `<main>${Navigation()}${children}</main>`;
}
```
