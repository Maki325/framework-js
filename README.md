# A JS Framework

## Goals

- [X] Parse simple JSX without Attributes
- [X] Parse custom JSX element calls without Attributes
- [X] Parse simple JSX with Attributes
- [X] Parse JSX so it can be streamed
- [ ] Optimize `JSXMemberExpr` to find `sync` components, so we don't treat them all as `async` and send them separately
- [ ] Escape HTML strings (maybe)
- [ ] Build the framework around the Server Components implementation
