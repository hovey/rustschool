use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Blocking a thread means that thread is prevented from
    // performing work or exiting. 
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Blocking a thread means that thread is prevented from
    // performing work or exiting. 
    handle.join().unwrap();
}
