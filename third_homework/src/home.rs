use std::{collections::{HashMap, HashSet}};
use crate::{provider::DeviceInfoProvider, errors::HomeError};

pub struct Home {
    pub rooms: HashMap<&'static str, Room>
}

impl Home {
    pub fn add_device(&mut self, room_name: &'static str, device_name: &'static str) -> Result<(), HomeError> {
        let _rn = room_name.to_owned();
        if let std::collections::hash_map::Entry::Occupied(_rn) = self.rooms.entry(room_name) {
            self.rooms.entry(room_name).and_modify(|f| f.add_device(device_name));
            Ok(())
        } else {
            Err(HomeError { code: 1000, message: "Room with provided name does not exist".to_string() })
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.name, room);
    }

    pub fn get_devices_ids(&self) -> Option<Vec<String>>  {
        if self.rooms.is_empty() {
            return None
        }
        let mut ids = Vec::new();
        for room in &self.rooms {
            for device in &room.1.devices {
                ids.push(device.to_owned().to_owned());
            }
        }
        Some(ids)
    }

    pub fn get_room_names(&self) -> Vec<&str> {
        let mut rooms = Vec::new();
        for room in &self.rooms {
            rooms.push(room.0.to_owned());
        }
        rooms
    }

    pub fn get_devices_names(&mut self, room_name: &'static str) -> Result<Vec<&str>, HomeError> {
        let _rn = room_name.to_owned();
        if let std::collections::hash_map::Entry::Occupied(_rn) = self.rooms.entry(room_name) {
            let room = self.rooms.get_key_value(room_name).unwrap();
            Ok(room.1.get_devices_names())
        } else {
            Err(HomeError { code: 1000, message: "Room with provided name does not exist".to_string() })
        }
    }

    pub fn create_report(&self, info_provider: &impl DeviceInfoProvider
    ) -> String {
        let ids = self.get_devices_ids().unwrap();
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

    pub fn get_devices_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        for device_name in self.devices.iter() {
            names.push(*device_name);
        }
        names
    }
}