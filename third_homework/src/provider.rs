use super::devices::{Socket, Thermometer, GetReport};

pub struct DevicesInfoProvider {
    pub so1200_01: Socket,
    pub the8200_01: Thermometer,
    pub so1200_02: Socket,
    pub the8200_02: Thermometer,
    pub _uy1200_01: Socket
}

pub trait DeviceInfoProvider {
    fn get_info(&self, ids: Vec<String>) -> String;
}

impl DeviceInfoProvider for DevicesInfoProvider {
    fn get_info(&self, ids: Vec<String>) -> String {
        let mut report = String::new();
        if ids.contains(&self.so1200_01.name.to_owned()) {
            report = report + &self.so1200_01.get_report();
        }
        if ids.contains(&self.so1200_02.name.to_owned()) {
            report = report + &self.so1200_02.get_report();
        }
        if ids.contains(&self.the8200_01.name.to_owned()) {
            report = report + &self.the8200_01.get_report();
        }
        if ids.contains(&self.the8200_02.name.to_owned()) {
            report = report + &self.the8200_02.get_report();
        }
        report
    }
} 
