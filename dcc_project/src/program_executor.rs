use std::thread;
use std::time::Duration;

use crate::{
    data_manipulator::DataManipulator,
    internal_communications::{InternalChannel, InternalMessage},
};
use uuid::Uuid;

pub struct ProgramExecutor {
    id: Uuid,
    channel_whith_mcm: InternalChannel<InternalMessage>,
    data_manipulator: DataManipulator,
}

impl ProgramExecutor {
    pub fn new(
        channel_whith_mcm: InternalChannel<InternalMessage>,
        data_manipulator: DataManipulator,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            channel_whith_mcm,
            data_manipulator,
        }
    }

    pub fn run(&self) {
        println!("PE[{}] up an running...", &self.id);
        for count in 1..10 {
            if let Ok(incomming) = self.channel_whith_mcm.rx.try_recv() {
                self.process_incomming_message(incomming);
            }
            thread::sleep(Duration::from_secs(1)); // simulate work
            let message = InternalMessage::Text {
                sender_id: self.id,
                payload: format!("Hello from PE/DM[{}]", &self.id),
            };
            let _ = self.channel_whith_mcm.tx.send(message);
        }
    }

    fn process_incomming_message(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => println!(
                "PE/DM[{}] received Text[{}] from the MCM[{}].",
                &self.id, p, i
            ),
        }
    }
}
