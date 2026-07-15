use std::sync::mpsc;
use std::thread;

use dcc_project::data_manipulator::DataManipulator;
use dcc_project::internal_communications::InternalChannel;
use dcc_project::management_and_control::ManagementControlMonitorization;
use dcc_project::program_executor::ProgramExecutor;

fn main() {
    let mut available_executors: Vec<InternalChannel> = Vec::new();
    for _ in 0..3 {
        available_executors.push(install_a_new_pe());
    }
    let mcm = thread::spawn(move || {
        ManagementControlMonitorization::new(available_executors).run();
    });
    match mcm.join() {
        Ok(m) => println!("The MCM finishes and says [{:?}]", m),
        Err(e) => println!("The MCM finishes and says [{:?}]", e),
    };
}

// I put install in the name of this function
// and not create or launch
// because in a real DCC system
// PE/DM subsystems will be hardware based ones.
fn install_a_new_pe() -> InternalChannel {
    let (tx_pe1, rx_mcm) = mpsc::channel();
    let (tx_mcm, rx_pe1) = mpsc::channel();
    let (tx_md, rx_pe2) = mpsc::channel();
    let (tx_pe2, rx_md) = mpsc::channel();
    let _ = thread::spawn(move || {
        let channel_pe_mcm = InternalChannel {
            tx: tx_pe1,
            rx: rx_pe1,
        };
        let channel_pe_md = InternalChannel {
            tx: tx_pe2,
            rx: rx_pe2,
        };
        ProgramExecutor::new(channel_pe_mcm, channel_pe_md).run();
    });
    let _ = thread::spawn(move || {
        let channel_md_pe = InternalChannel {
            tx: tx_md,
            rx: rx_md,
        };
        DataManipulator::new(channel_md_pe).run();
    });
    InternalChannel {
        tx: tx_mcm,
        rx: rx_mcm,
    }
}
