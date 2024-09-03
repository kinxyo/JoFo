mod core;

use core::basic_parser;

// WASM

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// #[wasm_bindgen]
// pub fn maybe() -> String {
//     String::from("WebAssembly on Nuxt! 🚀")
// }

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("JoFo")
}

#[wasm_bindgen]
pub fn formatter(content: String) -> JsValue {
    let parsed = basic_parser(content);
    serde_wasm_bindgen::to_value(&parsed).map_err(|e| e.into())

    /* match serde_json::to_string(&basic_parser(content)) {
        Ok(x) => x,
        Err(e) => {
            let err = format!("Problem with `formatter`: {}", e);
            String::from(err)
        }
    } */
    // println!("{:#?}", content);
    // String::from("hey")
}

#[cfg(test)]
mod tests {

    use super::{basic_parser, formatter};
    use crate::core::Token;

    #[test]
    fn basic_parsing() {
        let content = std::fs::read_to_string("parser/test.kinx").expect("unable to open file");

        let test_case = vec![
            Token {
                text: "this is bold text".to_string(),
                symbol_type: "bold".to_string(),
            },
            Token {
                text: "this is emphasizing".to_string(),
                symbol_type: "emphasize".to_string(),
            },
        ];
        assert_eq!(test_case, basic_parser(content));
    }

    #[test]
    fn formatter_working() {
        let content = std::fs::read_to_string("parser/test.kinx").expect("unable to open file");

        let test_case = String::from("value");

        let result = formatter(content);

        println!("{:#?}", result);

        assert_eq!(test_case, String::from("value"));
    }
}
