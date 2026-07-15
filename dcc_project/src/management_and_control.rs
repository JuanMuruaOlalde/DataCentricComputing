use std::thread;
use std::time::Duration;

use uuid::Uuid;

use crate::internal_communications::{InternalMessage, pe_dm_pair};
pub struct ManagementControlMonitorization {
    id: uuid::Uuid,
    available_executors: Vec<pe_dm_pair>,
}

impl ManagementControlMonitorization {
    pub fn new(available_executors: Vec<pe_dm_pair>) -> Self {
        Self {
            id: Uuid::new_v4(),
            available_executors,
        }
    }

    pub fn run(&self) {
        println!("The MCM up an running...");
        loop {
            for executor in &self.available_executors {
                if let Ok(incomming) = executor.pe.rx.try_recv() {
                    self.process_incomming_message(incomming);
                }
                if let Ok(incomming) = executor.dm.rx.try_recv() {
                    self.process_incomming_message(incomming);
                }
            }
            thread::sleep(Duration::from_secs(2)); // simulate work
            for executor in &self.available_executors {
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: String::from("heartbeat"),
                };
                let _ = &executor.pe.tx.send(message);
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: String::from("heartbeat"),
                };
                let _ = &executor.dm.tx.send(message);
            }
        }
    }

    fn process_incomming_message(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => println!("The MCM received a Text[{}] from [{}]", p, i),
        }
    }
}
