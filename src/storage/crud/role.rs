

use rusqlite::{Connection, Result, params, OpenFlags, named_params};
use crate::core::entities::return_data::ReturnData;
use crate::core::utils::config_parser::{
    Parser, CONFIG_PATH
};

pub struct RoleCRUD {
    conn: Connection
}

impl RoleCRUD {

    pub fn new(path: &String) -> Self {

        let connection = Connection::open_with_flags(path, OpenFlags::default())
            .map_err(|err| {
                println!("{:}", err);
            }).expect("[STORAGE.ERROR] Error occurred while connecting to storage (@role)");

        Self {
            conn: connection
        }
    }

    pub fn create_role(&self, role_name: &String, access_level: u16) -> Result<ReturnData, String> {

        let resul = self.conn.execute(
            "insert into Roles (name, access_level) values (:name, :access_level);",
            named_params! {
                ":name": role_name,
                ":access_level": access_level
            }
        ).map_err(|err| {
            format!("[STORAGE.ERROR] Can't add new role to database\n{}", err.to_string())
        })?;

        Ok(ReturnData::new(String::from("Role created successfully"), 1, String::from("")))
    }

    pub fn get_role_by_name(&self, role_name: &String) -> Result<ReturnData, String> {

        let mut receiver = self.conn
            .prepare("select (name, access_level) from Roles where name = :name;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get role with name {} from database\n{}", role_name, err.to_string())
            })?;

        let mut rows = receiver
            .query(named_params! {":name": role_name})
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query role name {} from database\n{}", role_name, err.to_string())
            })?;

        let mut value: String = String::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            value = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get role name from row\n{}", err.to_string()))?;
        }

        Ok(ReturnData::new(String::from("Role retrieved successfully"), 1, value))
    }

    pub fn get_all_roles(&self) -> Result<ReturnData, String> {
        let mut receiver = self.conn
            .prepare("select name from Roles;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get all roles from database\n{}", err.to_string())
            })?;

        let mut rows = receiver
            .query(())
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query all roles from database\n{}", err.to_string())
            })?;

        let mut raw_data: Vec<String> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| {
            format!("[STORAGE.ERROR] Can't get next role from rows\n{}", err.to_string())
        })? {
            raw_data.push(row.get(0).map_err(|err| {
                format!("[STORAGE.ERROR] Can't get role name from row\n{}", err.to_string())
            })?);
        }

        let data = raw_data.join("\n");
        let message = format!("Retrieved {} rows from database", raw_data.len());

        Ok(ReturnData::new(message, 1, data))
    }

    pub fn add_default_role(&self, user_id: u16) -> Result<(), String> {
        let parser = Parser::new(CONFIG_PATH)?;
        let raw_default_role_id = parser.get_value("DEFAULT_ROLE_ID")?;
        let default_role_id = raw_default_role_id.parse::<u16>()
            .map_err(|err| format!("[STORAGE.ERROR] Can't parse DEFAULT_ROLE_ID={} to u16", raw_default_role_id))?;

        let resul = self.conn.execute(
            "insert into UserRole (user_id, role_id) values (:user_id, :role_id);",
            named_params! {
                ":user_id": user_id,
                ":role_id": default_role_id
            }
        ).map_err(|err| {
            format!("[STORAGE.ERROR] Can't add role to user to database\n{}", err.to_string())
        })?;

        Ok(())
    }


}