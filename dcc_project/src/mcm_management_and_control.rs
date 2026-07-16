use uuid::Uuid;

use crate::internal_communications::{InternalMessage, pe_dm_pair};
use crate::user_interface;

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
        user_interface::display_on_screen("The MCM up an running...");
        loop {
            for executor in &self.available_executors {
                if let Ok(incomming) = executor.pe.rx.try_recv() {
                    self.process_incomming_message(incomming);
                }
                if let Ok(incomming) = executor.dm.rx.try_recv() {
                    self.process_incomming_message(incomming);
                }
            }
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
            match user_interface::ask_user(
                "Type a command for the DCC system: (`exit`[Enter] to terminate, [Enter] to continue) ",
            ) {
                Ok(command) => match command.as_str() {
                    "exit" | "EXIT" | "terminate" => {
                        self.end_program();
                        break;
                    }
                    _ => user_interface::display_on_screen(&format!(
                        "[{}] is not a valid command.",
                        command
                    )),
                },
                Err(error) => user_interface::display_on_screen(&format!("ERROR [{}]", error)),
            }
        }
    }

    fn process_incomming_message(&self, incomming: InternalMessage) {
        match incomming {
            InternalMessage::EndProgram { .. } => (), // loop break in run() function
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => user_interface::display_on_screen(&format!(
                "The MCM received a Text[{}] from [{}]",
                p, i
            )),
        }
    }

    fn end_program(&self) {
        for executor in &self.available_executors {
            user_interface::display_on_screen("Sending EndProgram to one of the PE/DM subsystems.");
            let message = InternalMessage::EndProgram { sender_id: self.id };
            let _ = &executor.pe.tx.send(message);
            let message = InternalMessage::EndProgram { sender_id: self.id };
            let _ = &executor.dm.tx.send(message);
        }
    }
}
