/*
Smart pointers in rust
code from video: https://www.youtube.com/watch?v=dYEC6NElVOg&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=23&ab_channel=Let%27sGetRusty
            
           

What are smart pointers ?

They are Datastructures that act like a pointer
but also have additional functionality added to it

rust smart pointer implement the deref traits and drop traits
*/


// deref if a shorthand for derefencing a pointer:
// let x = 5;
// let y = &x; //<----- y is a reference to x
// println!("{}", *y); //<---- derefencing the y pointer (getting the value to which y points)
// So what is happening under the hood ? a deref traits is implemented on the type.
// here we will show how it works

use std::ops::Deref;

// Here we are creating a sort of custom Box with generic types
// to store data
struct MyBox<T>(T);

// implementation of new method to create an instance of our struct
// but here x is not stored on the heap. We are just showcasing what the deref traits does
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// to dereference a MyBox ptr using the *var synthax we need to implement the deref trait
// Deref trait import is above
impl<T> Deref for MyBox<T> {
    // this is an associated type (don't worry about it)
    type Target = T;

    // returns a reference to the item in the tuple
    fn deref(&self) -> &T/*&Self::Target*/ {
        &self.0
    }
    // this function means that when we do *var, it will return a reference to the value contained
    // in var (we are effectively using the deref operator)
}

fn main() {

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // if we used *y on MyBox without the deref trait, this wouldn't work
                       // the goal was to implement the trait so we can use the operator instead of
                       // using y.0
    // println!("{}",y.0);
    // ^ Try to uncomment this line it does the same as *y



    // implicit Deref Coercions
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str
    // here we implemented the deref for immutable references
    // So you need to implement the DerefMut trait for mutable ref 

}

fn hello(name: &str) {
    println!("Hello {} !", name);
}
