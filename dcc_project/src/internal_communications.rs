use std::sync::mpsc::{Receiver, Sender};

pub struct InternalChannel {
    pub tx: Sender<InternalMessage>,
    pub rx: Receiver<InternalMessage>,
}

pub enum InternalMessage {
    Text {
        sender_id: uuid::Uuid,
        payload: String,
    },
}

impl std::fmt::Display for InternalMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InternalMessage::Text {
                sender_id: i,
                payload: p,
            } => write!(f, "{{payload: {}, sender_id: {}}}", p, i),
        }
    }
}
