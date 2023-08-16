enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

// enums can hold values:
enum Event {
    Load,
    KeyPress(char), // like tuple
    Click{x: i64, y: i64}, // or structs
}

// enums can also be type aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
// and then use in main
// let x = Operations::Add;
// but most often you'll see it on impl block with Self
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// there are also C like enums
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


//lastly, a more advanced usage of enum is in the box_pointers.rs file with a linked list example






fn main() {
    let player_direction:Direction = Direction::UP;

    match player_direction {
        Direction::UP => println!("heading UP"),
        Direction::DOWN => println!("heading DOWN"),
        Direction::LEFT => println!("heading LEFT"),
        Direction::RIGHT => println!("heading RIGHT"),
    }


    let event: Event = Event::KeyPress('x');
    match event {
        Event::KeyPress(c) => println!("User pressed key! `{}`!", c),
        _ => println!("Other event"),
    }

}
