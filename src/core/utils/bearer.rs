use crate::core::auth::entities::user::User;
use crate::core::utils::config_parser::{CONFIG_PATH, Parser};

pub fn admin_only(user: &User) -> Result<(), String>{
    let parser = Parser::new(CONFIG_PATH)?;
    let raw_value = parser.get_value("ADMIN_ACCESS_LEVEL")?;
    let admin_access_level = raw_value.parse::<u16>()
        .map_err(|err| format!("[FILE.ERROR] Can't cast ADMIN_ACCESS_LEVEL = {} to u16", raw_value))?;

    if (user.get_access_level() < admin_access_level) {
        Err(String::from("[ACCESS.ERROR] Permission denied.\n[ACCESS.ERROR] Your access level is not high enough!"))
    }
    Ok(())
}