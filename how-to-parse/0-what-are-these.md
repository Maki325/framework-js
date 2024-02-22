# What are these?

These are a **work in progress** abstract explanations of how I'll parse JSX files to implement `Server Components` myself.

It **will** change, but that doesn't matter to the end user as it's "private" code that no user should ever call directly, and will always be handled by the framework.

It's mostly here so I can better understand what I want to do, and how it should look in general, with steps that have **HUGE** gaps between them.

The current parsing only supports **not** async functions, which is why I know the parsing will change, as it will need to handle the async functions otherwise I can't really call these `Server Components`.

I will also try to implement the `'use client'` directive, but that's only after I finish the server-side stuff. And currently I don't have any idea how I'll do that, but that's a problem for future me.
