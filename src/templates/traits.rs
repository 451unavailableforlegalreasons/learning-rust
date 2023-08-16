

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet{
    // fn summarize(&self) -> String {
    //     return format!("{}, by {}", self.content, self.username)
    // }
}

pub trait Summary {
    // fn summarize (&self) -> String; // only declare the function do not code it
    // in case you want a default behavoir:
    fn summarize(&self) -> String {
        return "No Preview for object".to_string();
    }
}




pub fn notify(item: &impl Summary) { // something that implement the SUmmary traits (so we can call
                                     // its methods associated)
    println!("{}", item.summarize());
}

// you can also use generics
// pub fn notify<T: Summary>(item: &T) {
//      println!("{}", item.summarize());
// }



/*
You can also require mutiple traits like that:

pub fn notify(item: &(impl Summary + AnOtherTrait)) {
    ...
}



sometimes it can be a bit ugly so there is something called a where clause:
pub fn notify<T, U>(t: &T, u: &U) -> String
where T: Display + Clone, // which are traits
      U: Clone + Debug
{
    ... 
    function body here
    ...
}



*/


fn main () {

    let twit: Tweet = Tweet{
        username: "451".to_string(),
        content:"hell oworld".to_string(),
        reply:false,
        retweet:false
    };
    let art: NewsArticle = NewsArticle {
        author:"john".to_string(),
        headline: "bad day".to_string(),
        content: "nothing in here".to_string()
    };


    println!("Tweeter: {}", twit.summarize());
    println!("Articel: {}", art.summarize());
    notify(&twit);
    notify(&art);

}
