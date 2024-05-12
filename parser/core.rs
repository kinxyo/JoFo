/* symbol table */

#![allow(dead_code)]

enum SymbolType {
    BOLD,
    ITALIC,
    SIDENOTE,
    EMPHASIZE,
    HEADING1,
    HEADING2,
    HEADING3,
    HEADING4,
    HEADING5,
    HEADING6,
}

use std::collections::HashMap;

use serde::Serialize;

pub struct SymbolTable {
    symbols: HashMap<String, SymbolType>, // Key = syntax (formatting) | Value = symbol type
}

impl SymbolTable {
    pub fn init() -> Self {
        let mut symbols = SymbolTable {
            symbols: HashMap::new(),
        };

        symbols.symbols.insert("*".to_string(), SymbolType::BOLD);
        symbols.symbols.insert("_".to_string(), SymbolType::ITALIC);
        symbols
            .symbols
            .insert("!".to_string(), SymbolType::EMPHASIZE);
        symbols
            .symbols
            .insert("^".to_string(), SymbolType::SIDENOTE);
        symbols
            .symbols
            .insert("# ".to_string(), SymbolType::HEADING1);
        symbols
            .symbols
            .insert("## ".to_string(), SymbolType::HEADING2);
        symbols
            .symbols
            .insert("### ".to_string(), SymbolType::HEADING3);
        symbols
            .symbols
            .insert("#### ".to_string(), SymbolType::HEADING4);
        symbols
            .symbols
            .insert("##### ".to_string(), SymbolType::HEADING5);
        symbols
            .symbols
            .insert("###### ".to_string(), SymbolType::HEADING6);

        symbols
    }

    pub fn lookup(&self, syntax: &char) -> Option<String> {
        let s = syntax.to_string();
        match self.symbols.get(&s) {
            None => None,
            Some(SymbolType::BOLD) => Some(String::from("bold")),
            Some(SymbolType::ITALIC) => Some(String::from("italic")),
            Some(SymbolType::EMPHASIZE) => Some(String::from("emphasize")),
            Some(SymbolType::SIDENOTE) => Some(String::from("sidenote")),
            Some(SymbolType::HEADING1) => Some(String::from("h1")),
            Some(SymbolType::HEADING2) => Some(String::from("h2")),
            Some(SymbolType::HEADING3) => Some(String::from("h3")),
            Some(SymbolType::HEADING4) => Some(String::from("h4")),
            Some(SymbolType::HEADING5) => Some(String::from("h5")),
            Some(SymbolType::HEADING6) => Some(String::from("h6")),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Token {
    pub text: String,
    pub symbol_type: String,
}

pub fn basic_parser<'a>(content: String) -> Vec<Token> {
    let table: SymbolTable = SymbolTable::init();
    let mut inside_symbol: bool = false;
    let mut string = String::new();
    let mut result: Vec<Token> = Vec::new();

    for char in content.chars() {
        
        match inside_symbol {
            false => {

                match table.lookup(&char) {
                    None => continue,
                    Some(_) => {
                        inside_symbol = true;
                    }
                }
            },
            
            true => {
                match table.lookup(&char) {
                    None => {
                        string.push(char);
                    },
                    Some(x) => {
                        result.push(Token { text: string.clone(), symbol_type: x });
                        inside_symbol = false;
                        string.clear();
                    }
                }
                


            }

        }
        
        


    }
    result
    // content.lines().filter(|x| x.contains(&table.lookup(x).unwrap_or_default())).collect()
}
