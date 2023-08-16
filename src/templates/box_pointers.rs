/*
Smart pointers in rust

What are smart pointers ?

They are Datastructures that act like a pointer
but also have additional functionality added to it
*/

// ERROR: try uncommenting this enum
// enum List { // this enum has in infinite size so how do you use it ? 
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>), // here we force the next element to be stored on the heap
    Nil,
}
use List::{Cons, Nil};


fn main() {

    let b = Box::new(5); // Box pointer: 5 stored on the heap
    println!("{}", b);
/*
Use box pointers:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
*/

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // this is like a linked list
    // each next element (Cons or Nil is allocated on the heap - rust does not need to know the
    // size of the List before compiling, it will be on the heap and not the stack)

}
