

pub struct User {
    id: str,
    username: str,
    user_password: &UserPassword,
}

pub struct UserPassword {
    user_id: str,
    password_hash: str
}

impl UserPassword {

    fn compareHash(hash: str) -> bool {
        
    }

}