use zxcvbn::feedback::Warning::StraightRowsOfKeysAreEasyToGuess;
use crate::core::entities::hidden_prompt::HiddenPrompt;
use crate::core::entities::prompt::Prompt;
use crate::storage::storage::Storage;

struct InitUser(String, String);

pub fn initialize(db_path: &String) -> Result<(), String> {
    let user = register_root_user()?;
    let name = user.0;
    let hashed_password = user.1;

    init_db(&db_path.to_string());

    let storage = Storage::new(db_path.to_string());
    let auth_crud = storage.get_auth_crud();

    let result = auth_crud.create_user(&name, &hashed_password)?;
    let user_id = result.get_data().parse::<u16>().unwrap();

    let role_crud = storage.get_role_crud();
    role_crud.add_role(user_id, 1)?; // 1 - admin; maybe change hardcode later

    Ok(())
}

fn register_root_user() -> Result<InitUser, String> {
    let name = Prompt::new(&String::from("Username: ")).expect("Error occurred while reading input");
    let raw_password = HiddenPrompt::new(&String::from("Password: ")).expect("Error occurred while reading input");
    let submit_password = HiddenPrompt::new(&String::from("Repeat password: ")).expect("Error occurred while reading input");

    if raw_password.ne(&submit_password) { return Err(String::from("Passwords don't match!")); }

    let hashed_password = bcrypt::hash(raw_password, bcrypt::DEFAULT_COST).unwrap();

    Ok(InitUser(name, hashed_password))
}

fn init_db(path: &String) {
    let storage = Storage::new(path.to_string());
    storage.init();
}

