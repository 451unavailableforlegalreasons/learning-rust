/*
Smart pointers in rust
code from video: https://www.youtube.com/watch?v=77aRH6YBKyY&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=26&ab_channel=Let%27sGetRusty
           

What are smart pointers ?

They are Datastructures that act like a pointer
but also have additional functionality added to it

rust smart pointer implement the deref traits and drop traits
*/


/*


let a = 5;
let b = &mut a; // you can't do that (have a mutable ref to an immutable var)

let mut c = 10; 
let d = &c; // you can do that (have an immutable ref to a mutable var
*d = 20; // but obviously can't edit it 

/*
You could solve the last example with interior mutability 
*/
*/


fn main() {

}

