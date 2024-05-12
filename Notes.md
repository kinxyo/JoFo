hey !universe ^me^ hola !


// pub fn parse(content: String) -> Vec<Token> {
    // let table: SymbolTable = SymbolTable::init();
    // let mut tokens: Vec<Token> = Vec::new();
    // let mut text: String = String::new();
    // let mut track_text: Vec<String> = Vec::new();
    // let mut inside: bool = false;
    // let mut track_symbol: Vec<char> = Vec::new(); //stack

    /*

    Approach:
        func: collect all text until you find the same symbol again. (input: symbol)
              + ignore any other symbol.
        main: iterate over chars, find symbol? -> call func | find nested symbol? -> call func.
    */

    // let user_text: std::str::Chars = content.chars();

    // dont use below code:
    /* for char in content.chars() {
        match table.lookup(&char) {
            None => continue,
            Some(x) => {
                collect_text(user_text, x);
            }
        }
    } */

    // tokens
// }

// HELPER FUNCTIONS


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
