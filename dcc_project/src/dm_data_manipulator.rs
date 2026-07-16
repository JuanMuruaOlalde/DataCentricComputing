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
        let message = InternalMessage::Text {
            sender_id: self.id,
            payload: format!("MD[{}] up an running...", &self.id),
        };
        let _ = self.channel_whith_mcm.tx.send(message);

        loop {
            if let Ok(incomming) = self.channel_whith_mcm.rx.try_recv() {
                match incomming {
                    InternalMessage::EndProgram { .. } => break,
                    _ => self.process_message_from_mcm(incomming),
                }
            }
            if let Ok(incomming) = self.channel_whith_pe.rx.try_recv() {
                match incomming {
                    InternalMessage::EndProgram { .. } => (),
                    _ => self.process_message_from_pe(incomming),
                }
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
                    payload: format!("DM[{}] received Text[{}] from the MCM[{}].", &self.id, p, i),
                };
                let _ = self.channel_whith_mcm.tx.send(message);
            }
        }
    }
    fn process_message_from_pe(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::EndProgram { .. } => (), // loop break in run() function
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => {
                let message = InternalMessage::Text {
                    sender_id: self.id,
                    payload: format!(
                        "DM[{}] received Text[{}] from its associated PE[{}].",
                        &self.id, p, i
                    ),
                };
                let _ = self.channel_whith_mcm.tx.send(message);
            }
        }
    }
}
