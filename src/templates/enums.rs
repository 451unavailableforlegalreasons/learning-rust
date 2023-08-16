#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String), // you can put data in enums like this instead of using a separate struct
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));


    // println!("{:?} {:?}", four, six);
    let content = match localhost {
        V4 => V4,
        V6 => V6,
    };
    println!("{:?}", content);
}



