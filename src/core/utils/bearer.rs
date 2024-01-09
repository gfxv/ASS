use std::cmp::max;
use crate::core::auth::entities::user::User;
use crate::core::utils::config_parser::{CONFIG_PATH, Parser};
use crate::storage::storage::Storage;

pub fn admin_access(user: &User) -> Result<(), String>{
    let parser = Parser::new(CONFIG_PATH)?;
    let raw_value = parser.get_value("ADMIN_ACCESS_LEVEL")?;
    let admin_access_level = raw_value.parse::<u16>()
        .map_err(|err| format!("[FILE.ERROR] Can't cast ADMIN_ACCESS_LEVEL = {} to u16", raw_value))?;

    if (user.get_access_level() < admin_access_level) {
        handle_error()?;
    }
    Ok(())
}

pub fn mod_access(user: &User) -> Result<(), String> {
    let parser = Parser::new(CONFIG_PATH)?;
    let raw_value = parser.get_value("MOD_ACCESS_LEVEL")?;
    let mod_access_level = raw_value.parse::<u16>()
        .map_err(|err| format!("[FILE.ERROR] Can't cast MOD_ACCESS_LEVEL = {} to u16", raw_value))?;

    if (user.get_access_level() < mod_access_level) {
        handle_error()?;
    }
    Ok(())
}

pub fn default(db_path: &String, user: &User, password_name: &String) -> Result<(), String> {
    let storage = Storage::new(db_path.to_string());
    let password_crud = storage.get_password_crud();
    let group_crud = storage.get_group_crud();

    let password_id = password_crud.get_password_id_by_name(&password_name)?;
    let max_access_level = match group_crud.get_passwords_groups(password_id)?.iter().max_by_key(|&g| g.get_access_level()) {
        Some(group) => group.get_access_level(),
        None => 0u16
    };

    if user.get_access_level() < max_access_level {
        handle_error()?;
    }

    Ok(())
}

fn handle_error() -> Result<(), String> {
    Err(String::from("[ACCESS.ERROR] Permission denied.\n[ACCESS.ERROR] Your access level is not high enough!"))
}