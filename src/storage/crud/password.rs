use rusqlite::{Connection, Result, params, OpenFlags};


pub struct PasswordCRUD {
    conn: Connection
}


impl PasswordCRUD {

    pub fn new(path: &String) -> Self {
        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .expect("Error while connecting to storage");

        Self {
            conn: connection
        }
    }

    pub fn get_password_by_name(&self, name: String) {
        let password = self.conn.execute(
            "select value from Passwords where name = (:name)", 
            rusqlite::named_params! {":name" : name})
            .expect("Can't select password by name");

        println!("Password: {:?}", password);
    }

    // Note: probably should return status + message (?)
    pub fn insert_new_password(&self, name: String, value: String) {
        let result = self.conn.execute(
            "insert into Passwords (name, value) values (:name, :value)", 
            rusqlite::named_params! {
                ":name": name,
                ":value": value
            }).expect("Can't add new password to database");
            
        println!("Something works???");
    }

}