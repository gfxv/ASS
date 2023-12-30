
use rusqlite::{Connection, Result, params, OpenFlags};
use crate::core::entities::return_data::ReturnData;

pub struct GroupCRUD {
    conn: Connection
}

impl GroupCRUD {

    pub fn new(path: &String) -> Self {

        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .map_err(|err| {
                println!("{:}", err);
            }).expect("[STORAGE.ERROR] Error occurred while connecting to storage (@group)");

        Self {
            conn: connection
        }
    }

    pub fn get_all_groups(&self) -> ReturnData {
        let mut message = String::from("Group created successfully!");
        let mut status = 1;

        let mut receiver = self.conn
            .prepare("select name from Groups;")
            .map_err(|err| {
                message = err.to_string();
                status = 2;
            }).expect("[STORAGE.ERROR] Can't prepare statement to get all groups from database");

        let mut rows = receiver
            .query(())
            .map_err(|err| {
                message = err.to_string();
                status  = 2;
            }).expect("[STORAGE.ERROR] Can't query all groups from database");

        // ЫАЫААЫАЫАА КАСТЫЛИ ААЫАЫАЫА
        let mut raw_data: Vec<String> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| {
            message = err.to_string();
            status = 2;
        }).expect("[STORAGE.ERROR] Can't get next group from rows") {
            raw_data.push(row.get(0).map_err(|err| {
                message = err.to_string();
                status = 2;
            }).expect("[STORAGE.ERROR] Can't get group name from row"))
        }

        let data = raw_data.join("\n");

        ReturnData::new(message, status, data)

    }

    pub fn create_group(&self, group_name: &String, access_level: u16) -> Result<ReturnData, String> {

        let mut message = String::from("Group created successfully");
        let mut status = 1;

        let resul = self.conn.execute(
            "insert into Groups (name, access_level) values (:name, :access_level);",
            rusqlite::named_params! {
                ":name": group_name,
                ":access_level": access_level
            }
        ).map_err(|err| {
            format!("[STORAGE.ERROR] Can't add new group to database\n{}", err.to_string())
        })?;

        Ok(ReturnData::new(message, status, String::from("")))
    }

}