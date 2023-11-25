use rusqlite::{Connection, Result, params, OpenFlags};
use crate::core::entities::return_data::ReturnData;


pub struct PasswordCRUD {
    conn: Connection
}


impl PasswordCRUD {

    pub fn new(path: &String) -> Self {
        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .expect("[STORAGE.ERROR] Error while connecting to storage");

        Self {
            conn: connection
        }
    }

    pub fn get_password_by_name(&self, name: &String) -> ReturnData {

        let mut receiver = self.conn
            .prepare("select value from Passwords where name = :name")
            .expect("[STORAGE.ERROR] Can't prepare password selection");

        let mut rows = receiver
            .query(rusqlite::named_params!{ ":name": name})
            .expect("[STORAGE.ERROR] Can't query password");

        let mut value: String = String::new();
        while let Some(row) = rows.next().expect("[STORAGE.ERROR] Can't iterate through rows") {
            value = row.get(0).expect("[STORAGE.ERROR] Can't get password from row");
        }

        ReturnData::new(String::from(""), 1, value)
        // println!("Password: {:}", value);
    }

    // Note: probably should return status + message (?)
    pub fn insert_new_password(&self, name: &String, value: &String) -> ReturnData {
        let result = self.conn.execute(
            "insert into Passwords (name, value) values (:name, :value)", 
            rusqlite::named_params! {
                ":name": name,
                ":value": value
            }).expect("[STORAGE.ERROR] Can't add new password to database");
            
        ReturnData::new(String::from("Password added successfully"), 1, String::from(""))
    }

}