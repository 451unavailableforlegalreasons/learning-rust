/*
Concurrency
*/

use std::{thread, time::Duration};




fn main() {


    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // if we handle.join(); here, the main thread won't execute untill this thread is done

    for i in 0..5 {
        println!("{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("main thread terminated");
    handle.join().unwrap(); // if we don't write this line, the program exists because main thread
                            // is done
}




fn passing_values() {

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("here is a vector from main in a thread {:?}", v);
    });


    drop(v); // if v is dropped but used inside the thread, there is undefined behavior:
             // but since we asked the compiler to move the value v into the thread, we can't drop
             // it anymore in this main thread
    handle.join().unwrap();

}
