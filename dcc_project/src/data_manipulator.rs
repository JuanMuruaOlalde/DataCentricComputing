use std::thread;
use std::time::Duration;

use crate::internal_communications::{InternalChannel, InternalMessage};
use uuid::Uuid;

pub struct DataManipulator {
    id: Uuid,
    channel_whith_mcm: InternalChannel,
    channel_whith_pe: InternalChannel,
}

impl DataManipulator {
    pub fn new(channel_whith_mcm: InternalChannel, channel_whith_pe: InternalChannel) -> Self {
        Self {
            id: Uuid::new_v4(),
            channel_whith_mcm,
            channel_whith_pe,
        }
    }

    pub fn run(&self) {
        println!("MD[{}] up an running...", &self.id);
        loop {
            while let Ok(incomming) = self.channel_whith_mcm.rx.try_recv() {
                self.process_message_from_mcm(incomming);
            }
            while let Ok(incomming) = self.channel_whith_pe.rx.try_recv() {
                self.process_message_from_pe(incomming);
            }
            thread::sleep(Duration::from_secs(1)); // simulate work
        }
    }

    fn process_message_from_mcm(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => println!("MD[{}] received Text[{}] from the MCM[{}].", &self.id, p, i),
        }
    }
    fn process_message_from_pe(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => println!(
                "MD[{}] received Text[{}] from its associated PE[{}].",
                &self.id, p, i
            ),
        }
    }
}
