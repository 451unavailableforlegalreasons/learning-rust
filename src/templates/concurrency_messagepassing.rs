/*
Concurrency
*/

use std::{thread, time::Duration};
use std::sync::mpsc;




fn main() {
    
    let (tx, rx) = mpsc::channel(); // sender & receiver


    let handle = thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();
        // you can't use the msg var here because you let go of its ownership
    });


    let received = rx.recv().unwrap();
    // let received = rx.try_recv().unwrap(); // this method will not block the main thread
    // execution
    println!("received: {}", received);





    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let msg =vec![
            String::from("hello"),
            String::from("world"),
            String::from("hi"),
            String::from("mom"),
            String::from("it's me"),
        ];


        for m in msg {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msg =vec![
            String::from("oh "),
            String::from("this is"),
            String::from("the second"),
            String::from("thead ?"),
            String::from("wow !!"),
        ];


        for m in msg {
            tx2.send(m).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recevied in rx {
        println!("got: {}", recevied );
    }
   


}
