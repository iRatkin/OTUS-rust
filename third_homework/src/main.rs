use std::collections::{HashMap, HashSet};

use third_homework::{home::{Room, Home}, devices::{Socket, Thermometer}, provider::DevicesInfoProvider};

fn main() {
    let room = Room { name: "room", devices: HashSet::new() };
    let kitchen = Room { name: "kitchen", devices: HashSet::new() };
    let mut home = Home { rooms: HashMap::new() };

    home.add_room(room);
    home.add_room(kitchen);

    let so1200_01_name = "so1200_01";
    let so1200_02_name = "so1200_02";
    let the8200_01_name = "the8200_01";
    let the8200_02_name = "the8200_02";
    let uy1200_01_name = "uy1200_01";


    let so1200_01 = Socket {
        name: so1200_01_name,
        description: "Smart socket numba one",
        current_power: 10,
        state: true
    };

    let so1200_02 = Socket {
        name: so1200_02_name,
        description: "Smart socket numba tu",
        current_power: 42,
        state: true
    };

    let the8200_01 = Thermometer {
        name: the8200_01_name,
        description: "Smart thermo numba one",
        current_temp: 36
    };

    let the8200_02 = Thermometer {
        name: the8200_02_name,
        description: "Smart thermo numba tu",
        current_temp: 10
    };


    let _uy1200_01 = Socket {
        name: uy1200_01_name,
        description: "Thermo out of home",
        current_power: 42,
        state: true
    };

    let devices_info_provider = DevicesInfoProvider {
        so1200_01,
        the8200_01,
        so1200_02,
        the8200_02,
        _uy1200_01
    };

    home.add_device("kitchen", so1200_01_name);
    home.add_device("room", so1200_02_name);

    home.add_device("kitchen", the8200_01_name);
    home.add_device("room", the8200_02_name);


    let report1 = home.create_report(&devices_info_provider);
    println!("Report #1: {report1}");

}