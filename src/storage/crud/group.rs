
use rusqlite::{Connection, Result, params, OpenFlags, named_params};
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

    pub fn get_all_groups(&self) -> Result<ReturnData, String> {

        let mut status = 1;

        let mut receiver = self.conn
            .prepare("select name from Groups;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get all groups from database\n{}", err.to_string())
            })?;

        let mut rows = receiver
            .query(())
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query all groups from database\n{}", err.to_string())
            })?;

        // ЫАЫААЫАЫАА КАСТЫЛИ ААЫАЫАЫА
        let mut raw_data: Vec<String> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| {
            format!("[STORAGE.ERROR] Can't get next group from rows\n{}", err.to_string())
        })? {
            raw_data.push(row.get(0).map_err(|err| {
                format!("[STORAGE.ERROR] Can't get group name from row\n{}", err.to_string())
            })?);
        }

        let data = raw_data.join("\n");
        let message = format!("Retrieved {} rows from database", raw_data.len());

        Ok(ReturnData::new(message, status, data))

    }

    pub fn get_groups_by_access_level(&self, access_level: u16) -> Result<ReturnData, String> {

        let mut receiver = self.conn
            .prepare("select name from Groups where access_level = :level;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get groups with access level of {} from database\n{}", access_level, err.to_string())
            })?;

        let mut rows = receiver
            .query(named_params! { ":level": access_level})
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query groups with access level of {} from database\n{}", access_level, err.to_string())
            })?;

        let mut raw_data: Vec<String> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| {
            format!("[STORAGE.ERROR] Can't get next group from rows\n{}", err.to_string())
        })? {
            raw_data.push(row.get(0).map_err(|err| {
                format!("[STORAGE.ERROR] Can't get group name from row\n{}", err.to_string())
            })?);
        }

        let data = raw_data.join("\n");
        let message = format!("Retrieved {} rows from database", raw_data.len());

        Ok(ReturnData::new(message, 1, data))

    }

    pub fn get_group_by_name() {

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