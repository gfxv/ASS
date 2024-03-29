use std::ffi::CString;
use rusqlite::{Connection, Result, params, OpenFlags, named_params};
use crate::core::entities::return_data::ReturnData;


pub struct PasswordCRUD {
    conn: Connection
}


impl PasswordCRUD {

    pub fn new(path: &String) -> Self {
        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .expect("[STORAGE.ERROR] Error occurred while connecting to storage (@password)");

        Self {
            conn: connection
        }
    }

    pub fn get_password_by_name(&self, name: &String) -> Result<ReturnData, String> {

        let mut receiver = self.conn
            .prepare("select value from Passwords where name = :name")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare password selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query(rusqlite::named_params!{ ":name": name})
            .map_err(|err| format!("[STORAGE.ERROR] Can't query password\n{}", err.to_string()))?;

        let mut value: String = String::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            value = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get password from row\n{}", err.to_string()))?;
        }

        Ok(ReturnData::new(String::from(""), 1, value))

    }

    pub fn insert_new_password(&self, name: &String, value: &String) -> Result<ReturnData, String> {
        let result = self.conn.execute(
            "insert into Passwords (name, value) values (:name, :value)", 
            rusqlite::named_params! {
                ":name": name,
                ":value": value
            }
        ).map_err(|err| format!("[STORAGE.ERROR] Can't add new password to database\n{}", err.to_string()))?;
            
        Ok(ReturnData::new(String::from("Password added successfully"), 1, String::from("")))
    }

    // updates password value
    pub fn update_by_name(&self, name: &String, new_value: &String) -> Result<ReturnData, String> {

        let result = self.conn.execute(
            "update Passwords set value = :value where name = :name",
            rusqlite::named_params! {
                ":name": name,
                ":value": new_value
            }
        ).map_err(|err| format!("[STORAGE.ERROR] Can't update password value\n{}", err.to_string()))?;

        Ok(ReturnData::new(String::from("Password value updated successfully"), 1, String::from("")))
    }

    pub fn delete_password_by_name(&self, name: &String) -> Result<ReturnData, String> {

        let result = self.conn.execute(
            "delete from Passwords where name = :name",
            rusqlite::named_params! {
                ":name": name
            }
        ).map_err(|err| format!("[STORAGE.ERROR] Can't delete password\n{}", err.to_string()))?;

        Ok(ReturnData::new(String::from("Password was deleted successfully!"), 1, String::from("")))
    }

    pub fn password_exists(&self, resource_name: &String) -> Result<bool, String> {
        let mut receiver = self.conn
            .prepare("select id from Passwords where name = :resource_name;")
            .map_err(|err| format!("[STORAGE.ERROR] Can't prepare password id selection\n{}", err.to_string()))?;

        let mut rows = receiver
            .query_map(
                named_params!{ ":resource_name": resource_name},
                |row| row.get::<usize, usize>(0)
            ).map_err(|err| "[STORAGE.ERROR] Can't query password id\n".to_string())?;

        Ok(rows.count() != 0)
    }

    pub fn get_password_id_by_name(&self, resource_name: &String) -> Result<u16, String> {
        let mut receiver = self.conn
            .prepare("select id from Passwords where name = :resource_name;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get password with name {} from database\n{}", resource_name, err.to_string())
            })?;

        let mut rows = receiver
            .query(named_params! {":resource_name": resource_name})
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query password name {} from database\n{}", resource_name, err.to_string())
            })?;

        let mut value: i32 = 0i32;
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            value = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get password id from row\n{}", err.to_string()))?;
        }

        Ok(value as u16)
    }

}