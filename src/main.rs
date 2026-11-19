use std::thread::{self, JoinHandle};

fn main() {
    let mut elevator_tasks: Vec<JoinHandle<()>> = Vec::new();

    // Network Thread Tx
    elevator_tasks.push(thread::spawn(move || {

    }));

    // Network Thread Rx
    elevator_tasks.push(thread::spawn(move || {

    }));

    // Standby System Thread
    elevator_tasks.push(thread::spawn(move || {

    }));

    // Memory Controller Thread
    elevator_tasks.push(thread::spawn(move || {

    }));

    // Elevator Driver Thread
    elevator_tasks.push(thread::spawn(move || {

    }));

    for task in elevator_tasks {
        task.join().unwrap();
    }

    println!("Hello, world!");
}
