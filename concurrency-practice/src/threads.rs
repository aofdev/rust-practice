use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    run_thread_spawn();
    run_channel();
    run_shared_state();
}

// Thread with spawn
fn run_thread_spawn() {
    thread::spawn(|| {
        // let handle = thread::spawn(|| {
        for i in 1..10 {
            println!(
                "run number {} from the spawned thread! {:?}",
                i,
                thread::current().id()
            );
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!(
            "run number {} from the main thread! {:?}",
            i,
            thread::current().id()
        );
        thread::sleep(Duration::from_millis(1));
    }

    // Waiting for All Threads to Finish Using join Handles
    // handle.join().unwrap();
}

// Multi-producer, single-consumer FIFO queue communication primitives
fn run_channel() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hello channel");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // --------------------------------------------------------------
    // Sending Multiple Values and Seeing the Receiver Waiting
    // Multiple Producers
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("message1"),
            String::from("message2"),
            String::from("message3"),
            String::from("message4"),
        ];

        for val in vals {
            tx1.send(format!("{:?}|{}", thread::current().id(), val))
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("message5"),
            String::from("message6"),
            String::from("message7"),
            String::from("message8"),
            String::from("message9"),
        ];

        for val in vals {
            tx.send(format!("{:?}|{}", thread::current().id(), val))
                .unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Shared-State Concurrency
fn run_shared_state() {
    let (tx, rx) = mpsc::channel();

    let x = Arc::new(Mutex::new(tx));

    thread::spawn(move || {
        x.lock().unwrap().send(10u8).unwrap();
    });

    dbg!(rx.recv().unwrap());

    // -------------------------------------------------------------------
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!(
                "Current number: {} | {:?}",
                num.to_string(),
                thread::current().id()
            );
            *num += 10;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
