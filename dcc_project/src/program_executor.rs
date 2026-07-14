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
        println!("an PE up an running...");
        let message = InternalMessage::Text {
            sender_id: self.id,
            payload: format!("Hello from PE/DM subsystem with id[{}]", &self.id),
        };
        let _ = self.channel_whith_mcm.tx.send(message);
    }
}
