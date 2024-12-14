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
        mapping(U256=>Event) events;
        uint256 num_of_event;
    }

    pub struct Event{
        address owner;
        string title;
        string description;
        string date;
        mapping(uint256=>Attendees) attendees;
    }
    pub struct Attendees{
        uint256 id;
        string name;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl EventDB {
    pub fn CreateEvent(&mut self, title: String, description: String, date: String) {
        let num = self.num_of_event.get();
        let mut new_event = self.events.setter(num);
        new_event.owner.set(msg::sender());
        new_event.title.set_str(title);
        new_event.description.set_str(description);
        new_event.date.set_str(date);
        self.num_of_event.set(num + U256::from(1));
        format!(
            "
                
            ",
            new_event.title.get_string(),
            new_event.description.get_string(),
            new_event.date.get_string(),
        );
    }

    pub fn ListEvents(self) {
        let mut response = String::from("[");
        let max = self.num_of_event.get();
        // for i in 0..self.
        reponse.push_str("]");
    }
    pub fn Event($mut self, id: uint256){


    }
    pub fn RegisterEvent($mut self, id:uint256, name:String){
        // let event = self.nujevents.get(id)
        let num = self.num_of_event.get()

       let mut newAttendee =  event.attendees.setter(num + U256::from(1))
       newAttendee.id.set(id)
       newAttendee.name.set(name)
    //    self.num_of_event.set(num + U256::from(1));
    }
}
