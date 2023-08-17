/*
Smart pointers in rust
code from video: https://www.youtube.com/watch?v=M9Owp3iLigg&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=25&ab_channel=Let%27sGetRusty
           

What are smart pointers ?

They are Datastructures that act like a pointer
but also have additional functionality added to it

rust smart pointer implement the deref traits and drop traits
*/

/*
Reference counting pointers are counting instances of an object in order to delete it once nobody uses it
think of a garbage collected language. Each variable is allocated on the stack and every once in a while 
we check if no one is using it, if nobody used it, we delete it. If some people are still using it, we leave it.

the method used in this file is only useful for single threaded apps
*/

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};


fn main() {

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // this doesn't clone a but increment a counter inside Rc and
                                   // points to a
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(3, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


}

