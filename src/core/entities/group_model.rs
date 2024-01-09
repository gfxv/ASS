
pub struct Group {
    id: u16,
    name: String,
    access_level: u16
}

impl Group {

    pub fn new(id: u16, name: &String, access_level: u16) -> Self {
        Self {
            id: id,
            name: name.to_string(),
            access_level: access_level
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_access_level(&self) -> u16 {
        self.access_level
    }
}