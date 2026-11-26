use std::{thread::{self, JoinHandle}};

mod mutex_handler;
mod standby_system;
mod elevator_driver;
mod memory_controller;
mod network_driver;

fn main() {
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Thread
    elevator_tasks.push(thread::spawn(move || {
        network_driver::network_thread();
    }));

    // Standby System Thread
    elevator_tasks.push(thread::spawn(move || {
        loop {
            println!("standby")
        }
    }));

    // Memory Controller Thread
    elevator_tasks.push(thread::spawn(move || {
        loop {
            println!("mem")
        }
    }));

    // Elevator Driver Thread
    elevator_tasks.push(thread::spawn(move || {
        loop {
            println!("elev")
        }
    }));

    for task in elevator_tasks {
        task.join().unwrap();
    }
}
