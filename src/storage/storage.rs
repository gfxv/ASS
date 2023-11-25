
use rusqlite::{Connection, Result, params, OpenFlags};
use crate::storage::crud::{password::PasswordCRUD};


pub struct Storage {
    path: String,
    conn: Connection
}

impl Storage {

    pub fn new(path: String) -> Self {
        let connection = Connection::open_with_flags(&path, OpenFlags::default())
        .map_err(|e| {
            println!("{:}", e);
        }).expect("(check the path to database file)");

        connection.execute( "create table if not exists Users (
            id int primary key,
            username varchar(50) unique not null
        );", ()).expect("Error in creating Users table");

        connection.execute("create table if not exists AuthPasswords (
            id int primary key,
            user_id int,
            salt varchar(255) unique not null,
            password_hash varchar(255) not null,
            foreign key(user_id) references Users(id)
        );", ()).expect("Error in creating AuthPasswords table");

        connection.execute("create table if not exists Roles (
            id int primary key,
            name varchar(255) unique not null,
            access_level int not null
        );", ()).expect("Error in creating Roles table");

        connection.execute("create table if not exists UserRole (
            user_id int,
            role_id int,
            foreign key(user_id) references Users(id),
            foreign key(role_id) references Roles(id)
        );", ()).expect("Error in creating UserRole table");

        connection.execute( "create table if not exists Passwords (
            id int primary key,
            name varchar(50) unique not null,
            value varchar(50) not null
        );", ()).expect("Error in creating Passwords table");

        connection.execute( "create table if not exists Groups (
            id int primary key,
            name varchar(50) unique not null,
            access_level int not null
        );", ()).expect("Error in creating Groups table");

        connection.execute( "create table if not exists PasswordGroup (
            password_id int,
            group_id int,
            foreign key (password_id) references Passwords(id),
            foreign key (group_id) references Groups(id)
        );", ()).expect("Error in creating PasswordGroup table");

        
        let storage = Storage { 
            path: path,
            conn: connection
        };

        storage
    }

    pub fn get_password_crud(&self) -> PasswordCRUD {
        PasswordCRUD::new(&self.path)
        // PasswordCRUD::new(self.get_connection())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

}