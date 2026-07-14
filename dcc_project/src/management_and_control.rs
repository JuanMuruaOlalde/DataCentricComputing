use uuid::Uuid;

use crate::internal_communications::{InternalChannel, InternalMessage};
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
                while let Ok(incomming) = executor.rx.try_recv() {
                    self.process_incomming_message(incomming);
                }
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: String::from("heartbeat"),
                };
                let _ = &executor.tx.send(message);
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
