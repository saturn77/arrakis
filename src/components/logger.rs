
use crate::models::details::Details;

pub struct Logger {
    pub logger_text: String,

}

impl Default for Logger {
    fn default() -> Self {
        Self {
            logger_text: String::new(),
        }
    }
}

impl Logger {

    pub fn clear(&mut self) -> String {
        "".to_string()
    }

    pub fn system_info(&mut self) -> String{
        Details::new().format_os()
    }

    pub fn get_fpga_version(&mut self) -> String{
        "Requesting version info from FPGA".to_string()
    }

    #[allow(dead_code)]
    pub fn append(&mut self, text: &str) {
        self.logger_text.push_str(text);
    }

}



 
