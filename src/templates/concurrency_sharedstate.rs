/*
Concurrency - sharing state
*/

use std::{thread, time::Duration};
use std::sync::Mutex;
use std::sync::Arc;
// use std::rc::Rc;

/*
I'm not going to explain how locking works...
but we are going to use mutex to implement it.
*/


fn main() {
    let counter = Arc::new(Mutex::new(0)); // even though counter is immutable, Mutex implement
                                           // interior mutability. So we can still edit counter
                                           // content

    let mut handles = vec![];

    for _ in 0..10 {
        //shadow the counter var in the for looop scope
        // let counter = Rc::clone(&counter); // we cannot use the rc pointer to pass data between
                                           // threads, so we'll use atomic
        let counter = Arc::clone(&counter);
        handles.push(
            thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num +=1;
            })    
        );
    }

    for handle in handles {
        handle.join().unwrap();
    }





    println!("{:?}", counter);
}
