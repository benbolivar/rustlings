// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(Vec::<u128>::new()));

    for i in 0..10 {
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);

            let mut results = results.lock().unwrap();
            results.push(start.elapsed().as_millis());
        }));
    }

    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        handle.join().unwrap();
    }

    if results.lock().unwrap().len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.lock().unwrap().iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
