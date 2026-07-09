use crate::{
    data_manipulator::DataManipulator,
    utils::{InternalChannel, InternalMessage},
};

pub struct ProgramExecutor {
    channel_whith_mcm: InternalChannel<InternalMessage>,
    data_manipulator: DataManipulator,
}

impl ProgramExecutor {
    pub fn new(
        channel_whith_mcm: InternalChannel<InternalMessage>,
        data_manipulator: DataManipulator,
    ) -> Self {
        Self {
            channel_whith_mcm,
            data_manipulator,
        }
    }

    pub fn run(&self) {
        println!("an PE up an running...");
        let _ = self.channel_whith_mcm.tx.send(InternalMessage {
            payload_text: Some(String::from("Hello from an PE/DM subsystem")),
            payload_integer: None,
            payload_float: None,
        });
    }
}
