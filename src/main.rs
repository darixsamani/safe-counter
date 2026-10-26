use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            for _ in 0..1000{
                let mut num = c.lock().unwrap();
                *num += 1;
            }

        });
        handles.push(h);
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("The value of counter after 10 interations is: {}", *counter.lock().unwrap())
}

