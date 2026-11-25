use std::{thread::{self, JoinHandle}, time::Duration};

mod mutex_handler;
mod standby_system;
mod elevator_driver;
mod memory_controller;
mod network_driver;

use crate::network_driver::{tcp_handler::tcp_handler::TcpServer, udp_handler::udp_handler::UdpServer};

fn main() {
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Thread
    elevator_tasks.push(thread::spawn(move || {
        network_driver::network_thread();
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
