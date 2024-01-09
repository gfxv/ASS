
use rusqlite::{Connection, Result, OpenFlags, named_params};
use crate::core::entities::return_data::ReturnData;
use crate::core::entities::role_model::Role;
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

    pub fn get_role_id_by_name(&self, role_name: &String) -> Result<ReturnData, String> {

        let mut receiver = self.conn
            .prepare("select id from Roles where name = :name;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get role with name {} from database\n{}", role_name, err.to_string())
            })?;

        let mut rows = receiver
            .query(named_params! {":name": role_name})
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query role name {} from database\n{}", role_name, err.to_string())
            })?;

        let mut value: i32 = 0i32;
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't iterate through rows\n{}", err.to_string()))? {
            value = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get role id from row\n{}", err.to_string()))?;
        }

        Ok(ReturnData::new(String::from("Role id retrieved successfully"), 1, value.to_string()))
    }

    pub fn get_users_roles(&self, user_id: u16) -> Result<Vec<Role>, String> {

        let mut receiver = self.conn
            .prepare("
            select role.id, role.name, role.access_level from Roles role
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

        let mut data: Vec<Role> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't get next role from rows\n{}", err.to_string()))? {
            let raw_id: i16 = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get role id from row\n{}", err.to_string()))?;
            let name: String = row.get(1).map_err(|err| format!("[STORAGE.ERROR] Can't get role name from row\n{}", err.to_string()))?;
            let raw_access_level: i16 = row.get(2).map_err(|err| format!("[STORAGE.ERROR] Can't get role access level from row\n{}", err.to_string()))?;

            data.push(Role::new(raw_id as u16, &name, raw_access_level as u16));
        }


        Ok(data)
    }

    pub fn get_all_roles(&self) -> Result<ReturnData, String> {
        let mut receiver = self.conn
            .prepare("select id, name, access_level from Roles;")
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't prepare statement to get all roles from database\n{}", err.to_string())
            })?;

        let mut rows = receiver
            .query(())
            .map_err(|err| {
                format!("[STORAGE.ERROR] Can't query all roles from database\n{}", err.to_string())
            })?;

        let mut raw_data: Vec<String> = Vec::new();
        while let Some(row) = rows.next().map_err(|err| format!("[STORAGE.ERROR] Can't get next role from rows\n{}", err.to_string()))? {
            let raw_id: i16 = row.get(0).map_err(|err| format!("[STORAGE.ERROR] Can't get role id from row\n{}", err.to_string()))?;
            let name: String = row.get(1).map_err(|err| format!("[STORAGE.ERROR] Can't get role name from row\n{}", err.to_string()))?;
            let raw_access_level: i16 = row.get(2).map_err(|err| format!("[STORAGE.ERROR] Can't get role access level from row\n{}", err.to_string()))?;
            raw_data.push(format!("[{}] {} (access level = {})", raw_id, name, raw_access_level));
        }

        let data = raw_data.join("\n");
        let message = format!("Retrieved {} rows from database", raw_data.len());

        Ok(ReturnData::new(message, 1, data))
    }

    pub fn add_default_role(&self, user_id: u16) -> Result<u16, String> {
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

        Ok(default_role_id)
    }

    pub fn add_role(&self, user_id: u16, role_id: u16) -> Result<usize, String>{

        let result = self.conn.execute(
            "insert into UserRole (user_id, role_id) values (:user_id, :role_id);",
            named_params! {
                ":user_id": user_id,
                ":role_id": role_id
            }
        ).map_err(|err| {
            format!("[STORAGE.ERROR] Can't add role to user to database\n{}", err.to_string())
        })?;

        // Ok(ReturnData::new(String::from("Role successfully added!"), 1, String::new()))
        Ok(result)
    }

    pub fn remove_role(&self, user_id: u16, role_id: u16) -> Result<usize, String> {

        let result = self.conn.execute(
            "delete from UserRole where user_id = :user_id and role_id = :role_id",
            named_params! {
                ":user_id": user_id,
                ":role_id": role_id
            }
        ).map_err(|err| {
            format!("[STORAGE.ERROR] Can't remove role (role id = {}) from user (user id = {})", role_id, user_id)
        })?;

        Ok(result)
    }

}