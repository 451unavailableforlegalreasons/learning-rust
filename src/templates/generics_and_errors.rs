//results store Ok and Err
/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
struct Point<T, U> {
    x: T,
    y: U,
}
// generic type struct, can be different types because T and U (ex: T=i32, U=f64)
// let point = Point{x:5,y:13.2};
// you can use traits to restrict what can go trough (see this in traits file)
// generally for a function you do: fn myFunc<T: traits>(arg: T, arg2: T) -> T {}


impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x 
    }
}

use std::fs::File;

fn main () {

    let f = File::open("hello.txt");


    let f = match f {
        Ok(file) => file,
        Err(error) => {
            println!("Error: {:?}", error);
            panic!("exiting");
        }
    };


    panic!("Quits the programs with an error message");
}
