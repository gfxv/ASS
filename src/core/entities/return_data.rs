use std::ffi::CString;

/// `message` - status or error message
///
/// `status` - status code (in progress...)
///
/// `data` - retreived data (empty string if error occurred or operation doesn't return any data)
pub struct ReturnData {
    message: String,
    status: i16,
    data: String
}

impl ReturnData {

    pub fn new(message: String, status: i16, data: String) -> Self {
        Self { 
            message, 
            status, 
            data
        }
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }

    pub fn get_status(&self) -> &i16{
        &self.status
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    // pub fn set_message(&self, msg: &String) {
    //     &self.message = msg;
    // }

    pub fn set_status(mut self, status: i16) {
        self.status = status;
    }

    pub fn set_data(&self, data: &String) {

    }

}