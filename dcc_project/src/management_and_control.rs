use core::option::Option::None;
use uuid::Uuid;

use crate::utils::{InternalChannel, InternalMessage};

pub struct ManagementControlMonitorization {
    id: uuid::Uuid,
    available_executors: Vec<InternalChannel<InternalMessage>>,
}

impl ManagementControlMonitorization {
    pub fn new(available_executors: Vec<InternalChannel<InternalMessage>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            available_executors,
        }
    }

    pub fn run(&self) {
        println!("The MCM up an running...");
        loop {
            for executor in &self.available_executors {
                for incomming in &executor.rx {
                    self.process_incomming_message(incomming);
                }
                let _ = &executor.tx.send(InternalMessage {
                    sender_id: self.id,
                    payload_text: Some(String::from("heartbeat")),
                    payload_integer: None,
                    payload_float: None,
                });
            }
        }
    }

    fn process_incomming_message(&self, incomming: InternalMessage) {
        println!("The MCM received [{}]", incomming);
    }
}
