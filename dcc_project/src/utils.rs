use core::fmt::Formatter;
use std::{
    ffi::os_str::Display,
    sync::mpsc::{Receiver, Sender},
};

pub struct InternalChannel<InternalMessage> {
    pub tx: Sender<InternalMessage>,
    pub rx: Receiver<InternalMessage>,
}

pub struct InternalMessage {
    pub payload_text: Option<String>,
    pub payload_integer: Option<i32>,
    pub payload_float: Option<f64>,
}

impl std::fmt::Display for InternalMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{text: {}, integer: {}, float: {}}}",
            match &self.payload_text {
                Some(t) => t.to_string(),
                None => String::from("none"),
            },
            match &self.payload_integer {
                Some(i) => i.to_string(),
                None => String::from("none"),
            },
            match &self.payload_float {
                Some(f) => f.to_string(),
                None => String::from("none"),
            }
        )
    }
}
