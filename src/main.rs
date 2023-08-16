/*
Smart pointers in rust
code from video: https://www.youtube.com/watch?v=M9Owp3iLigg&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=25&ab_channel=Let%27sGetRusty
           

What are smart pointers ?

They are Datastructures that act like a pointer
but also have additional functionality added to it

rust smart pointer implement the deref traits and drop traits
*/


// the drop trait is what happend when te pointer goes out of scope
// rust will automatically free the ressource for you. (If they have a drop trait)


struct CustomSmartPointer {
    data: String,
}

// use std::ops::Drop; // this is automatically imported by rust

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // this is useful because you can release a lock (when using threading...) when dropping a
        // pointer
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("custom smart pointer where created");

    //here both values will go out of scope.
    //the drop method we implemented will be executed


    // you could mannualy drop c or d
    // drop(_c);
}

