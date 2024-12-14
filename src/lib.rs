//!
//! Stylus Hello World
//!
//! The following contract implements the Counter example from Foundry.
//!
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```
//!
//! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
//! To do this, run `cargo stylus export-abi`.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
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
        int id;
        uint256 num_of_attendes;
        mapping(uint256 => address) attendees;
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
        self.num_of_event.set(num + U256::from(1));
        format!(
            "{{\"title\":\"{}\",\"description\":\"{}\",\"date\":\"{}\",\"date\":\"{}\"}}",
            new_event.title.get_string(),
            new_event.description.get_string(),
            new_event.date.get_string(),
            new_event.id.get(),
        );
    }

    pub fn list_all(&self) {
        let mut response = String::from("[");
        let max_str = self.num_of_event.to_string();
        let max = max_str.parse().unwrap();
        for i in 0..max {
            let event = self.events.get(U256::from(i));
            let str = format!(
                "{{\"title\":\"{}\",\"description\":\"{}\",\"date\":\"{}\",\"date\":\"{}\"}},",
                event.title.get_string(),
                event.description.get_string(),
                event.date.get_string(),
                event.id.get(),
            );
            response.push_str(str.as_str());
            if i != max {
                response.push_str(",");
            }
        }
        response.push_str("]");

        response;
    }
}
