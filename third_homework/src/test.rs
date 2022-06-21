

#[cfg(test)]
mod test_home {
    use std::collections::{HashSet, HashMap};

    use crate::{home::{Room, Home}};
    #[test]
    fn get_rooms() {
        let room_name = "room";
        let kitchen_name = "kitchen";

        let room = Room { name: room_name, devices: HashSet::new() };
        let kitchen = Room { name: kitchen_name, devices: HashSet::new() };
        let mut home = Home { rooms: HashMap::new() };

        home.add_room(room);
        home.add_room(kitchen);

        let room_names = home.get_room_names();
        let expect = vec![room_name, kitchen_name];
        let matching = room_names.iter().zip(expect.iter()).filter(|(room_names, expect)| expect == room_names).count();
        assert_eq!(matching, room_names.len());
        assert_eq!(matching, expect.len());
    }

    #[test]
    fn unique_room() {
        let room_name = "room";

        let first_room = Room { name: room_name, devices: HashSet::new() };
        let second_room = Room { name: room_name, devices: HashSet::new() };
        let mut home = Home { rooms: HashMap::new() };

        home.add_room(first_room);
        home.add_room(second_room);

        let rooms = home.get_room_names();

        assert_eq!(rooms.len(), 1);
    }

    #[test]
    fn get_devices() {
        let room_name = "room";
        let device_name = "socket";
        let room = Room { name: room_name, devices: HashSet::new() };
        let mut home = Home { rooms: HashMap::new() };

        home.add_room(room);
        home.add_device(room_name, device_name).unwrap();
        home.add_device(room_name, device_name).unwrap();

        let devices_names = home.get_devices_names(room_name).unwrap();

        assert_eq!(devices_names.len(), 1);
    }
}