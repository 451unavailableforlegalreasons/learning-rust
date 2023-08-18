/*
   Object oriented features
*/



// this is kinda Polymorphism
struct Screen {
    components: Vec<Box<dyn Draw>>, // dyn is for dynamic dispatch
    // static dispatch = the compiler knows which function you are calling at runtime - (faster but
    // cannot add new components
    // dynamic dispatch = the compiler doesn't know - (slower but you can implement new components
    // at runtime)
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

trait Draw {
    fn draw(&self);
}


struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button with label {} | w:{},h:{}", self.label, self.width, self.height);
    }
}



struct CheckBox {
    width: u32,
    height: u32,
    option: String,
}

impl Draw for CheckBox{
    fn draw(&self) {
        println!("Drawing checkbox with option {} | w:{},h:{}", self.option, self.width, self.height);
    }
}

fn main() {

    let screen = Screen {
        components: vec![
            Box::new(CheckBox{
                width: 100,
                height: 100,
                option: "Hello World".to_string(),
            }),
            Box::new(Button{
                width:10,
                height:10,
                label: "blah blah blah".to_string(),
            })
        ],
    };



    screen.run();


}
