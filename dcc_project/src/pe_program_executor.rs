use crate::internal_communications::{InternalChannel, InternalMessage};
use uuid::Uuid;

pub struct ProgramExecutor {
    id: Uuid,
    channel_whith_mcm: InternalChannel,
    channel_whith_dm: InternalChannel,
}

impl ProgramExecutor {
    pub fn new(channel_whith_mcm: InternalChannel, channel_whith_dm: InternalChannel) -> Self {
        Self {
            id: Uuid::new_v4(),
            channel_whith_mcm,
            channel_whith_dm,
        }
    }

    pub fn run(&self) {
        let message = InternalMessage::Text {
            sender_id: self.id,
            payload: format!("PE[{}] up an running...", &self.id),
        };
        let _ = self.channel_whith_mcm.tx.send(message);

        loop {
            if let Ok(incomming) = self.channel_whith_mcm.rx.try_recv() {
                match incomming {
                    InternalMessage::EndProgram { .. } => break,
                    _ => self.process_message_from_mcm(incomming),
                };
            };
            if let Ok(incomming) = self.channel_whith_dm.rx.try_recv() {
                match incomming {
                    InternalMessage::EndProgram { .. } => (),
                    _ => self.process_message_from_dm(incomming),
                };
            }
        }
    }

    fn process_message_from_mcm(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::EndProgram { .. } => (), // loop break in run() function
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => {
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: format!("PE[{}] received Text[{}] from the MCM[{}].", &self.id, p, i),
                };
                let _ = self.channel_whith_mcm.tx.send(message);
            }
        }
    }
    fn process_message_from_dm(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::EndProgram { .. } => (), // loop break in run() function
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => {
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: format!(
                        "PE[{}] received Text[{}] from its associated DM[{}].",
                        &self.id, p, i
                    ),
                };
                let _ = self.channel_whith_mcm.tx.send(message);
            }
        }
    }
}
