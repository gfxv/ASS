use std::fmt::format;
use rusqlite::{Connection, Result, params, OpenFlags, named_params};
use crate::core::entities::return_data::ReturnData;

use crate::core::auth::entities::user::User;

/// For tables:
/// Users, AuthPasswords
pub struct AuthCRUD {
    conn: Connection
}


impl AuthCRUD {

    pub fn new(path: &String) -> Self {

        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .map_err(|err| {
                println!("{:}", err);
            }).expect("[STORAGE.ERROR] Error occurred while connecting to storage (@auth)");

        Self {
            conn: connection
        }
    }

    pub fn create_user(&self, username: &String, hash_password: &String) -> Result<ReturnData, String> {
        let result = self.conn.execute(
            "insert into Users (username) values (:username)",
            named_params! {
                ":username": username,
            }
        ).map_err(|err| format!("[STORAGE.ERROR] Can't add new user to database\n{}", err.to_string()))?;

        let user_id = self.get_user_id_by_name(username)?;
        self.add_password(user_id, hash_password)?;

        Ok(ReturnData::new(String::from("User created successfully"), 1, String::from(user_id.to_string())))
    }

    /// NO HASHING PERFORMED HERE, ONLY INSERTING TO DB
    fn add_password(&self, user_id: u16, password: &String) -> Result<(), String> {
        let result = self.conn.execute(
            "insert into AuthPasswords (user_id, password_hash) values (:user_id, :password_hash)",
            named_params! {
                ":user_id": user_id,
                ":password_hash": password
            }
        ).map_err(|err| format!("[STORAGE.ERROR] Can't add auth password to database\n{}", err.to_string()))?;

        Ok(())
    }

    fn get_user_id_by_name(&self, username: &String) -> Result<u16, String> {

        let mut receiver = self.conn
            .prepare("select id from Users where username = :username;")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare user id selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query(named_params!{ ":username": username})
            .map_err(|err| format!("[STORAGE.ERROR] Can't query user id\n{}", err.to_string()))?;


        let mut raw_id:i64 = 0;
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            raw_id = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get user id from row\n{}", err.to_string()))?;
        }

        Ok(raw_id as u16)
    }

    pub fn user_already_exists_by_username(&self, username: &String) -> Result<bool, String> {
        let mut receiver = self.conn
            .prepare("select id from Users where username = :username;")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare user id selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query_map(
                named_params!{ ":username": username},
                |row| row.get::<usize, usize>(0)
            ).map_err(|err| "[STORAGE.ERROR] Can't query user id\n".to_string())?;

        Ok(rows.count() != 0)
    }

    pub fn get_user_by_name(&self, username: &String) -> Result<User, String> {
        let mut receiver = self.conn
            .prepare("select id, username from Users where username = :username;")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare user selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query(rusqlite::named_params!{ ":username": username})
            .map_err(|err| format!("[STORAGE.ERROR] Can't query user\n{}", err.to_string()))?;

        let mut user_id:i64 = 0;;
        let mut username = String::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            user_id = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get user id from row\n{}", err.to_string()))?;
            username = row.get(1).map_err(|err| format!("[STORAGE.ERROR] Can't get user name from row\n{}", err.to_string()))?;
        }

        // let user_id = raw_id.parse::<u16>()
        //     .map_err(|err| format!("[STORAGE.ERROR] Can't convert user id {} to u16\n{}", raw_id, err.to_string()))?;

        let password_hash = self.get_auth_password_by_user_id(user_id as u16)?;

        let max_access_level = match self.get_roles_als_by_user_id(user_id as u16)?.iter().max() {
            Some(level) => level.to_owned(),
            None => 0u16
        };

        let user = User::new(user_id as u16, username, password_hash, max_access_level);

        Ok(user)

    }

    fn get_auth_password_by_user_id(&self, user_id: u16) -> Result<String, String> {
        let mut receiver = self.conn
            .prepare("select password_hash from AuthPasswords where user_id = :user_id;")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare password selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query(rusqlite::named_params!{ ":user_id": user_id})
            .map_err(|err| format!("[STORAGE.ERROR] Can't query password\n{}", err.to_string()))?;

        let mut password_hash = String::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            password_hash = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get password hash from row\n{}", err.to_string()))?;
        }

        Ok(password_hash)
    }

    /// For UserRole Table (Many-to-Many relation)
    /// als = access levels
    fn get_roles_als_by_user_id(&self, user_id: u16) -> Result<Vec<u16>, String> {

        let mut receiver = self.conn
            .prepare("
            select role.access_level from Roles role
            join UserRole ur on role.id = ur.role_id
            where ur.user_id = :user_id;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get all roles for specific user from database\n{}", err.to_string())
            })?;

        let mut rows = receiver
            .query(named_params! {":user_id": user_id as i64})
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query all roles for specific user from database\n{}", err.to_string())
            })?;

        let mut raw_ids: Vec<i64> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| {
            format!("[STORAGE.ERROR] Can't get next role from rows\n{}", err.to_string())
        })? {
            raw_ids.push(row.get(0).map_err(|err| {
                format!("[STORAGE.ERROR] Can't get role id from row\n{}", err.to_string())
            })?);
        }

        let ids: Vec<u16> = raw_ids.into_iter().map(|id| id as u16).collect();

        Ok(ids)
    }

}