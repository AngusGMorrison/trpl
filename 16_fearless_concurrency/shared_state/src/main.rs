use std::cmp;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn main() {
    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Arc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // deadlocks();

    no_deadlock();
}

fn deadlocks() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    {
        let lock1 = lock1.clone();
        let lock2 = lock2.clone();
        thread::spawn(move || {
            let result1 = lock1.lock();
            println!("acquired lock1 from spawned thread");
            let result2 = lock2.lock();
            println!("acquired lock2 from spawned thread");
        });
    }

    let result2 = lock2.lock();
    println!("acquired lock2 from main thread");
    let result1 = lock1.lock();
    println!("acquired lock1 from main thread");
}

fn no_deadlock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    {
        let lock1 = lock1.clone();
        let lock2 = lock2.clone();
        thread::spawn(move || {
            let result = lock_both(&lock1, &lock2);
            println!("acquired lock1 from spawned thread");
            println!("acquired lock2 from spawned thread");
        });
    }

    let result = lock_both(&lock1, &lock2);
    println!("acquired lock1 from main thread");
    println!("acquired lock2 from main thread");
}

fn lock_both<'a, T>(
    x: &'a Arc<Mutex<T>>,
    y: &'a Arc<Mutex<T>>,
) -> (MutexGuard<'a, T>, MutexGuard<'a, T>) {
    let x_addr = Arc::as_ptr(&x);
    let y_addr = Arc::as_ptr(&y);
    println!("x_addr = {:?}, y_addr = {:?}", x_addr, y_addr);

    let min = cmp::min(x_addr, y_addr);

    let guard1: MutexGuard<T>;
    let guard2: MutexGuard<T>;
    if min == x_addr {
        guard1 = x.lock().unwrap();
        guard2 = y.lock().unwrap();
    } else {
        guard1 = y.lock().unwrap();
        guard2 = x.lock().unwrap();
    }

    return (guard1, guard2);
}
