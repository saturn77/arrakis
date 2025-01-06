
use super::super::*;

#[derive(Default, Debug)]
pub struct Banner {
    pub message: String,
}

impl Banner {
    pub fn new() -> Banner {
        Banner {
            message: String::new(),
        }
    }

    pub fn format(&mut self) {
        self.message = format!("\n**** Welcome to Vescript FPGA Manager, Version {}", parameters::gui::VERSION); 
        self.message += &format!("\n**** Today is {}\n", chrono::Utc::now().format("%m-%d-%Y %H:%M:%S"));
        
    }

    pub fn print(&mut self) {
        println!("{}", self.message);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_banner() {
        let mut banner = super::Banner::new();
        banner.format();
        banner.print();
    }
}