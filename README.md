# Notes-app

(primarily personal use)

## ideas

- use rust for document handling, use webassembly, buwuhahahah!

## Generate the `WASM` file

```bash
wasm-pack build --target web --out-dir ./path/to/output
```

## Challenege

okay so, I think the reason why my wasm file is serving on development server but not in static build, is because of server-side rendering. WASM wrapper is for client-side and not server, so in development server, I am easily able to run it at client side because the client has to render the code involving WASM, however, during the `generate` command in Nuxt; where static build is created, the webapp is pre-rendered via server and because WASM wrapper is not compatible server-side, so it ignores it.
