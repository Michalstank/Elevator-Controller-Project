use std::thread::{self, JoinHandle};

use crate::network_driver::udp_handler::udp_handler::UdpServer;

mod mutex_handler;
mod standby_system;
mod network_driver;
mod elevator_driver;
mod memory_controller;

fn main() {
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Thread Tx
    elevator_tasks.push(thread::spawn(move || {
        use network_driver::{tcp_handler, udp_handler};
        let udp: UdpServer = UdpServer::spawn();
        
    }));

    // Network Thread Rx
    elevator_tasks.push(thread::spawn(move || {
        use network_driver;
    }));

    // Standby System Thread
    elevator_tasks.push(thread::spawn(move || {
        use standby_system;
    }));

    // Memory Controller Thread
    elevator_tasks.push(thread::spawn(move || {
        use memory_controller;
    }));

    // Elevator Driver Thread
    elevator_tasks.push(thread::spawn(move || {
        use elevator_driver;
    }));

    for task in elevator_tasks {
        task.join().unwrap();
    }
}
