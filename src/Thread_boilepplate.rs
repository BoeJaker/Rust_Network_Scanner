use std::thread;

fn main() {
    let num_threads = 4;

    let mut threads = Vec::new();

    for i in 0..num_threads {
        let thread = thread::spawn(move || {
            println!("Thread {} started", i);
            // Your thread logic goes here
            println!("Thread {} finished", i);
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().expect("Failed to join a thread");
    }

    println!("All threads finished");
}