use std::collections::{HashMap, HashSet};
use crate::provider::DeviceInfoProvider;

pub struct Home {
    pub rooms: HashMap<&'static str, Room>
}

impl Home {
    pub fn add_device(&mut self, room_name: &'static str, device_name: &'static str) {
        let room = match self.rooms.entry(room_name) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(_v) => panic!("There is no room with such name"),
        };
        room.add_device(device_name);
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name, room);
    }

    pub fn get_devices_ids(&self) -> Vec<String>  {
        let mut ids = Vec::new();
        for room in &self.rooms {
            // println!("{:?}", room.1.devices);
            for device in &room.1.devices {
                ids.push(device.to_owned().to_owned());
            }
        }
        ids
    }

    pub fn create_report(&self, info_provider: &impl DeviceInfoProvider
    ) -> String {
        let ids = self.get_devices_ids();
        info_provider.get_info(ids)
    }
}

pub struct Room {
    pub name: &'static str,
    pub devices: HashSet<&'static str>
}

impl Room {
    pub fn add_device(&mut self, device: &'static str) {
        self.devices.insert(device);
    }
}