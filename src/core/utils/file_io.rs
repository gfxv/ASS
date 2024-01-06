use std::env::consts::OS;
use std::fs::read_to_string;
use std::path::Path;
use crate::core::utils::config_parser::{
    Parser, CONFIG_PATH
};



const WIN_KEY_PATH: &str = "WIN_KEY_PATH";
const MAC_KEY_PATH: &str = "MAC_KEY_PATH";
const LNX_KEY_PATH: &str = "LNX_KEY_PATH";

pub fn get_cypher_key_path() -> Result<String, String> {

    let path: String;
    let parser = Parser::new(CONFIG_PATH)?;

    match OS {
        "windows" => path = parser.get_value(&WIN_KEY_PATH)?.to_string(),
        "macos" => path = parser.get_value(&MAC_KEY_PATH)?.to_string(),
        "linux" => path = parser.get_value(&LNX_KEY_PATH)?.to_string(),
        _ => {
            return Err(String::from("[FILE.ERROR] OS not supported"));
        }
    }

    Ok(path)
}

pub fn get_file_data(path: &String) -> Result<String, String> {
    let data = read_to_string(path)
        .map_err(|err| format!("[FILE.ERROR] Can't read file {}\n{}", path, err.to_string()))?;

    Ok(data)
}