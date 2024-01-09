
use rusqlite::{Connection, Result, params, OpenFlags};
use crate::storage::crud::{
    password::PasswordCRUD,
    group::GroupCRUD,
    role::RoleCRUD
};
use crate::storage::crud::auth::AuthCRUD;


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

        let storage = Storage { 
            path: path,
            conn: connection
        };

        storage
    }

    pub fn init(&self) {
        self.conn.execute( "create table if not exists Users (
            id integer primary key autoincrement,
            username varchar(50) unique not null
        );", ()).expect("Error in creating Users table");

        self.conn.execute("create table if not exists AuthPasswords (
            id integer primary key autoincrement,
            user_id int,
            password_hash varchar(255) not null,
            foreign key(user_id) references Users(id)
        );", ()).expect("Error in creating AuthPasswords table");

        self.conn.execute("create table if not exists Roles (
            id integer primary key autoincrement,
            name varchar(255) unique not null,
            access_level int not null
        );", ()).expect("Error in creating Roles table");

        self.conn.execute("create table if not exists UserRole (
            user_id int,
            role_id int,
            foreign key(user_id) references Users(id),
            foreign key(role_id) references Roles(id)
        );", ()).expect("Error in creating UserRole table");

        self.conn.execute( "create table if not exists Passwords (
            id integer primary key autoincrement,
            name varchar(50) unique not null,
            value varchar(50) not null
        );", ()).expect("Error in creating Passwords table");

        self.conn.execute( "create table if not exists Groups (
            id integer primary key autoincrement,
            name varchar(255) unique not null,
            access_level int not null
        );", ()).expect("Error in creating Groups table");

        self.conn.execute( "create table if not exists PasswordGroup (
            password_id int,
            group_id int,
            foreign key(password_id) references Passwords(id),
            foreign key(group_id) references Groups(id)
        );", ()).expect("Error in creating PasswordGroup table");


        self.conn.execute("
            insert or ignore into Roles (name, access_level) values
                ('admin', 100),
                ('mod', 90),
                ('user', 10),
                ('no', 0);
        ", ()).expect("Failed to create roles");
    }

    pub fn get_password_crud(&self) -> PasswordCRUD {
        PasswordCRUD::new(&self.path)
    }

    pub fn get_group_crud(&self) -> GroupCRUD {
        GroupCRUD::new(&self.path)
    }

    pub fn get_role_crud(&self) -> RoleCRUD {
        RoleCRUD::new(&self.path)
    }

    pub fn get_auth_crud(&self) -> AuthCRUD {AuthCRUD::new(&self.path)}

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

}