use sysinfo::System;
use local_ip_address::local_ip;

#[derive(Default)]
pub struct Details {
    pub name           : String, 
    pub kernel         : String, 
    pub version        : String, 
    pub host_name      : String, 
    pub physical_cores : String, 
    pub threaded_cores : String, 
    pub mem_used       : String, 
    pub mem_avail      : String, 
    pub mem_total      : String, 
    pub cpu_brand      : String,
    pub cpu_freq       : String,
    pub ip_addr        : String, 
}

impl Details {
    pub fn new() -> Details {
        Details::default()
    }

    pub fn get_ip(&mut self) {
        match local_ip() {
            Ok(ip) => self.ip_addr = format!("{}", ip),
            Err(err) => self.ip_addr = format!("Failed to get ip address {}", err),
        };
    }

    pub fn get_os(&mut self) {

        // Please note that we use "new_all" to ensure that all list of
        // components, network interfaces, disks and users are already
        // filled!
        let mut sys = System::new_all();

        // First we update all information of our `System` struct.
        sys.refresh_all();


        match local_ip() {
            Ok(ip) => self.ip_addr = format!("{}", ip),
            Err(err) => self.ip_addr = format!("Failed to get ip address {}", err),
        };


        if let Some(alpha) = System::name() {
            self.name = format!("{alpha}");
        }
        if let Some(alpha) = System::kernel_version() {
            self.kernel = format!("{alpha}");
        }
        if let Some(alpha) = System::os_version() {
            self.version = format!("{alpha}");
        }
        if let Some(alpha) = System::host_name() {
            self.host_name = format!("{alpha}");
        }
        if let Some(alpha) = sys.physical_core_count() {
            self.physical_cores = format!("{alpha}");
        }
        self.threaded_cores = format!("{}",sys.cpus().len());
        self.mem_avail = format!("{:.2} GB",(sys.available_memory() as f32)/1e9);

        self.mem_used = format!("{:.2} GB",(sys.used_memory() as f32)/1e9);
        self.mem_total = format!("{:.2} GB",(sys.total_memory() as f32/1e9));
        //self.cpu_brand = format!("{}",sys.cpus().brand());
        for cpu in sys.cpus() {
            self.cpu_brand = format!("{}", cpu.brand());
        }
        for cpu in sys.cpus() {
            self.cpu_freq = format!("{:.2} GHz", (cpu.frequency() as f32)/1e3);
        }

    }

    pub fn print_os(&mut self) {
        let sys = System::new_all();
        self.get_os();
        println!("System Name      = {}", self.name);
        println!("System Kernel    = {}", self.kernel);
        println!("Version          = {}", self.version);
        println!("Host Name        = {}", self.host_name);
        println!("Physical Cores   = {}", self.physical_cores);
        println!("Threaded Cores   = {}", sys.cpus().len());
        println!("Total memory     = {}", self.mem_total);
        println!("Available Memory = {}", self.mem_avail);
        println!("Used Memory      = {}", self.mem_used);
        println!("CPU Vendor       = {}", self.cpu_brand);
        println!("CPU Frequency    = {}", self.cpu_freq);
    }

    pub fn format_os(&mut self) -> String {
        let sys = System::new_all();
        self.get_os();
        let mut os : String = "".to_string();
        self.get_ip();

        os += &format!("System Name      = {}\r\n", self.name);
        os += &format!("System Kernel    = {}\r\n", self.kernel);
        os += &format!("Version          = {}\r\n", self.version);
        os += &format!("Host Name        = {}\r\n", self.host_name);
        os += &format!("\r\n");
        os += &format!("Physical Cores   = {} \r\n", self.physical_cores);
        os += &format!("Threaded Cores   = {} \r\n", sys.cpus().len());
        os += &format!("\r\n");
        os += &format!("Total memory     = {} \r\n", self.mem_total);
        os += &format!("Available Memory = {} \r\n", self.mem_avail);
        os += &format!("Used Memory      = {} \r\n", self.mem_used);
        os += &format!("\r\n");
        os += &format!("CPU Frequency    = {} \r\n", self.cpu_freq);
        os += &format!("CPU Vendor       = {} \r\n", self.cpu_brand);
        os += &format!("IP Address       = {}",self.ip_addr);
        
        os
    }

}


#[cfg(test)]
mod test {
    #[test]
    fn print_os() {
        let mut info = super::Details::new();
        println!("***************************");
        println!("*** Getting System Information ....");
        println!(); 
        info.get_os(); 
        info.print_os();
        println!();
        println!("{}",info.format_os());
        
    }
}