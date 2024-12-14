#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
sol_storage! {
    #[entrypoint]
    pub struct EventDB {
        mapping(uint256 => Event) events;
        uint256 num_of_event;
    }

    pub struct Event{
        address owner;
        string title;
        string description;
        string date;
        uint256 id;
        mapping(uint256=>Attendees) attendees;
        uint256 num_of_attendees;
    }
    pub struct Attendees{
        uint256 id;
        string name;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl EventDB {
    pub fn create_event(&mut self, title: String, description: String, date: String) {
        let num = self.num_of_event.get();
        let mut new_event = self.events.setter(num);
        new_event.owner.set(msg::sender());
        new_event.title.set_str(title);
        new_event.description.set_str(description);
        new_event.date.set_str(date);
        new_event.id.set(num);
        self.num_of_event.set(num + U256::from(1));
    }

    pub fn list_all(&self) -> String {
        let mut response = String::from("[");
        let max_str = self.num_of_event.to_string();
        let max = max_str.parse().unwrap();
        for i in 0..max {
            let event = self.events.get(U256::from(i));
            let str = format!(
                "{{\"title\":\"{}\",\"description\":\"{}\",\"date\":\"{}\",\"id\":\"{}\",\"owner\":\"{}\"}},",
                event.title.get_string(),
                event.description.get_string(),
                event.date.get_string(),
                event.id.get(),
                event.owner.get()
            );
            response.push_str(str.as_str());
        }
        response.push_str("]");

        response
    }
    // pub fn get_event(&mut self, id: U256) -> String {
    //     let event = self.events.get(id);
    //     let response = format!(
    //         "{{\"title\":\"{}\",\"description\":\"{}\",\"date\":\"{}\",\"id\":\"{}\",}}",
    //         event.title.get_string(),
    //         event.description.get_string(),
    //         event.date.get_string(),
    //         event.id.get(),
    //     );
    //     response
    // }

    pub fn register_event(&mut self, id: U256, name: String) {
        let mut event = self.events.setter(id);
        let num = event.num_of_attendees.get();

        let mut new_attendee = event.attendees.setter(num);
        new_attendee.id.set(num);
        new_attendee.name.set_str(name);
        event.num_of_attendees.set(num + U256::from(1));
    }

    pub fn list_attendees(&self, id: U256) -> String {
        let event = self.events.get(id);
        let mut response = String::from("[");
        let max_str = event.num_of_attendees.to_string();
        let max = max_str.parse().unwrap();
        for i in 0..max {
            let attendee = event.attendees.get(U256::from(i));
            let str = format!(
                "{{\"id\":\"{}\",\"name\":\"{}\"}},",
                attendee.id.get(),
                attendee.name.get_string()
            );
            response.push_str(str.as_str());
        }
        response.push_str("]");
        response
    }
}
