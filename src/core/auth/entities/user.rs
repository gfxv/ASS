
use bcrypt::verify;

#[derive(Clone)]
pub struct User {
    id: u16,
    username: String,
    user_password: String,
    access_level: u16
}

impl User {

    pub fn new(
        id: u16, username: String, user_password: String, access_level: u16
    ) -> Self {

        Self {
            id, username, user_password, access_level
        }
    }

    // (?) Change -> bool to Result<...>
    pub fn compare_passwords(&self, other_password: &String) -> bool {
        verify(other_password, &self.user_password).unwrap()
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.user_password
    }

    pub fn get_access_level(&self) -> u16 {
        self.access_level
    }
}