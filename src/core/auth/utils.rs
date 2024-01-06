use std::fmt::format;
use std::io::SeekFrom::Start;
use crate::cli::utils::parse_user_input;
use crate::core::auth::entities::user::User;
use crate::core::entities::{
    hidden_prompt::HiddenPrompt,
    prompt::Prompt
};

use crate::storage::storage::Storage;

pub fn auth(storage_path: &String) -> Result<String, String> {

    let action = Prompt::new(&String::from("Register or login? (r/l) "))
        .map_err(|err| format!("[AUTH.ERROR] {}", err.to_string()))?
        .to_lowercase().trim().to_string();


    if action.eq_ignore_ascii_case("r") {
        return Ok(register(storage_path)?);
    } else if action.eq_ignore_ascii_case("l") {
        return Ok(login(storage_path)?);
    }

    return Err(String::from("Wrong input!"))

}

pub fn register(storage_path: &String) -> Result<String, String> {

    let input_name = Prompt::new(&String::from("Username: "))
        .map_err(|err| format!("[CORE.ERROR] Can't parse username input\n{}", err.to_string()))?;
    let input_password = HiddenPrompt::new(&String::from("Password: "))
        .map_err(|err| format!("[CORE.ERROR] Can't parse password input\n{}", err.to_string()))?;

    let storage = Storage::new(storage_path.to_string());

    let auth_crud = storage.get_auth_crud();

    if auth_crud.user_already_exists_by_username(&input_name)? {
        return Err(String::from("[AUTH.ERROR] User already exists"));
    }

    let hashed_password = bcrypt::hash(input_password, bcrypt::DEFAULT_COST).unwrap();
    let result = auth_crud.create_user(&input_name, &hashed_password)?;

    let user_id = result.get_data().parse::<u16>().unwrap();

    let role_crud = storage.get_role_crud();
    role_crud.add_default_role(user_id)?;


    Ok(format!("{}\nYou logged in as {}", result.get_message().to_string(), input_name))
}

pub fn login(storage_path: &String) -> Result<String, String> {

    let input_name = Prompt::new(&String::from("Username: "))
        .map_err(|err| format!("[CORE.ERROR] Can't parse username input\n{}", err.to_string()))?;
    let input_password = HiddenPrompt::new(&String::from("Password: "))
        .map_err(|err| format!("[CORE.ERROR] Can't parse password input\n{}", err.to_string()))?;

    let auth_crud = Storage::new(storage_path.to_string()).get_auth_crud();

    if !auth_crud.user_already_exists_by_username(&input_name)? {
        return Err(String::from("[AUTH.ERROR] No such user"));
    }

    let user = auth_crud.get_user_by_name(&input_name)
        .map_err(|err| format!("[AUTH.ERROR] An error occurred\n{}", err.to_string()))?;

    if !user.compare_passwords(&input_password) {
        return Err(String::from("[AUTH.ERROR] Wrong password"));
    }


    Ok(String::from(format!("You logged in as {}", input_name)))
}
