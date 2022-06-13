pub struct Socket {
    pub name: &'static str,
    pub description: &'static str,
    pub current_power: i32,
    pub state: bool,
}

pub struct Thermometer {
    pub name: &'static str,
    pub description: &'static str,
    pub current_temp: i32,
}

pub trait GetReport {
    fn get_report(&self) -> String;
}

impl GetReport for Socket {
    fn get_report(&self) -> String {
        let report = format!(
            "\nName: {}, Current power: {}, State: {}, Description: {}",
            self.name,
            self.current_power,
            self.state,
            self.description
        );
        report
    }
}

impl GetReport for Thermometer {
    fn get_report(&self) -> String {
        let report = format!(
            "\nName: {}, Current temperature: {}, Description: {}",
            self.name,
            self.current_temp,
            self.description
        );
        report
    }
}
