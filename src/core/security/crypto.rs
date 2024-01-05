use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use crate::core::utils::file_io::{
    get_cypher_key_path, get_file_data
};

pub fn encrypt_data(data: &String) -> Result<String, String> {
    let key = get_password_key()?;
    let magic = new_magic_crypt!(key, 256);
    let encrypted = magic.encrypt_str_to_base64(data);
    Ok (encrypted)
}

pub fn decrypt_data(data: &String) -> Result<String, String> {
    let key = get_password_key()?;
    let magic = new_magic_crypt!(key, 256);
    let decrypted = magic.decrypt_base64_to_string(data).unwrap();
    Ok(decrypted)
}

fn get_password_key() -> Result<String, String> {
    let key_path = get_cypher_key_path()?;
    let key = get_file_data(&key_path)?;
    Ok(key)
}