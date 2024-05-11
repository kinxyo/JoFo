mod parser;

use parser::Token;

use parser::parse;

use std::fs::{self, read_to_string};
use std::path::PathBuf;

pub fn filefind(path: PathBuf, extension: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // If the entry is a file and it has the correct extension, add it to the list
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() && entry.path().extension() == Some(extension.as_ref()) {
                        let file_name = entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned();
                        files.push(file_name);
                    }
                }
            }
        }
    }
    files
}

pub fn parser(file: &String) -> Vec<Token> {
    let content = read_to_string(file).expect("unable to open file");

    println!("{}", content);

    let result = parse(content);

    println!("{:#?}", result);

    result
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    fn parsing_test() {
        /* testing findfiles */
        let current_dir = env::current_dir().expect("Failed to get current directory");

        let files_present = filefind(current_dir, "kinx");

        assert_eq!(files_present, ["file.kinx"]);

        // TESTCASE #1
        let test_case = vec![
            Token {
                text: "universe".to_string(),
                symbol_type: "emphasize".to_string(),
            },
            Token {
                text: "kinjalk".to_string(),
                symbol_type: "sidenote".to_string(),
            },
        ];
        assert_eq!(parser(&files_present[0]), test_case);

        // TESTCASE #2
        // let test_case = vec![
        //     Token { text: "universe".to_string(), symbol_type: "emphasize".to_string() },
        //     Token { text: "kinjalk".to_string(), symbol_type: "sidenote".to_string() }
        // ];
        // assert_eq!(parser(&files_present[0]), test_case);
    }
}


// WASM

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn greets() -> String {
    "hellos".to_string()
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}