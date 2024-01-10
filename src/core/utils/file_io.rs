use std::fs::read_to_string;
use crate::core::utils::config_parser::{
    Parser, CONFIG_PATH
};



const KEY_PATH: &str = "KEY_PATH";

pub fn get_cypher_key_path() -> Result<String, String> {

    let parser = Parser::new(CONFIG_PATH)?;
    let path  = parser.get_value(&KEY_PATH)?.to_string();

    Ok(path)
}

pub fn get_file_data(path: &String) -> Result<String, String> {
    let data = read_to_string(path)
        .map_err(|err| format!("[FILE.ERROR] Can't read file {}\n{}", path, err.to_string()))?;

    Ok(data)
}