#[allow(dead_code)]


#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    French,
}


fn main() {

    let language = Language::English;

    match language {
        Language::English => println!("this is English"),
        Language::Spanish => println!("this is Spanish"),
        Language::Russian => println!("this is Russian"),
        Language::French => println!("this is French"),
        _ => println!("unknonwn - never printed"),
        lang => println!("unsported language: {:?}", lang),
    }






    let authorizatoin_statu: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<i8, _> = "34".parse();

    if let Some(status) = authorizatoin_statu {
    println!("authorization status: {}", status);
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("privileged");
        } else {
            println!("basic");
        }
    } else { // this else isn't forced by the compiler and can be deleted
        println!("guest");
    }




    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);


    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    let v = vec![1,2,3];

    for (index, value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }


    let x = 5;
    let (x,y,z) = (1,2,3);
    let (x,y,_) = (1,2,3); // ignoring last value


    let point = (3,5);
    print_coord(&point);



    // refutability:
    // ireffutable
    let x = 5;

    //refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // can only accept irrefutable patterns:
    // function params
    // let statement
    // for loops

    // you cannot do this
    let x = Option<&str> = None;
    let Some() = x;
        
}

fn print_coord(&(x,y): &(i32,i32)) {
    println!("{} {}", x, y);
}
