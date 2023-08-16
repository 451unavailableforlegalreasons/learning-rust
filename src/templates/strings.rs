fn main() {


    let mut a = vec![];
    let b = vec![1,2,3];
    a.push(12);



    println!("{:?}", a);
    println!("{}", a.contains(&12));
    a.extend(b);
    a.resize(2, 0);
    println!("{:?}",a);




    let strlit: &str = "bowser";
    let mut chaine: String = String::from("aaahahah");
    println!("{}", format!("{} {}", chaine, strlit));
    println!("{}", chaine.find("ah").unwrap_or(1)); 
    
    chaine = format!("    {}   ", chaine);
    println!("chaine: {}", chaine.trim());
    chaine = chaine.trim().to_string();
    println!("chaine: {}", chaine);
    println!("{} {}", chaine.starts_with("aa"), chaine.ends_with("h"));
    println!("{}", chaine.get(1..).unwrap());
    println!("{:?}", chaine.chars());


    panic!("panic test");
}



