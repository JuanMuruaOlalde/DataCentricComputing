use core::fmt::Alignment::Left;
use core::option::Option::None;
use std::sync::mpsc;
use std::thread;

use dcc_project::data_manipulator::DataManipulator;
use dcc_project::management_and_control::ManagementControlMonitorization;
use dcc_project::program_executor::ProgramExecutor;
use dcc_project::utils::{InternalChannel, InternalMessage};

fn main() {
    let mut available_executors: Vec<InternalChannel<InternalMessage>> = Vec::new();
    let (tx_pe, rx_mcm) = mpsc::channel();
    let (tx_mcm, rx_pe) = mpsc::channel();
    let channel01 = InternalChannel {
        tx: tx_mcm,
        rx: rx_mcm,
    };
    thread::spawn(move || {
        let channel = InternalChannel {
            tx: tx_pe,
            rx: rx_pe,
        };
        ProgramExecutor::new(channel, DataManipulator {}).run();
    });
    available_executors.push(channel01);
    let (tx_pe, rx_mcm) = mpsc::channel();
    let (tx_mcm, rx_pe) = mpsc::channel();
    let channel02 = InternalChannel {
        tx: tx_mcm,
        rx: rx_mcm,
    };
    thread::spawn(move || {
        let channel = InternalChannel {
            tx: tx_pe,
            rx: rx_pe,
        };
        ProgramExecutor::new(channel, DataManipulator {}).run();
    });
    available_executors.push(channel02);
    let (tx_pe, rx_mcm) = mpsc::channel();
    let (tx_mcm, rx_pe) = mpsc::channel();
    let channel03 = InternalChannel {
        tx: tx_mcm,
        rx: rx_mcm,
    };
    thread::spawn(move || {
        let channel = InternalChannel {
            tx: tx_pe,
            rx: rx_pe,
        };
        ProgramExecutor::new(channel, DataManipulator {}).run();
    });
    available_executors.push(channel03);
    let mcm = thread::spawn(move || {
        ManagementControlMonitorization::new(available_executors).run();
    });
    match mcm.join() {
        Ok(m) => println!("The MCM finishes and says [{:?}]", m),
        Err(e) => println!("The MCM finishes and says [{:?}]", e),
    };
}
