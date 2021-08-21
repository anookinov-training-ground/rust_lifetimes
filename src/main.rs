#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime elision rules - lifetime annotation patterns programmed into Rust's analysis of references
// 3 lifetime rules
// 1) each parameter that is a reference gets its own lifetime parameter
// i.e. fn first_word(s: &str) -> &str {}
// i.e. fn first_word<'a>(s: &'a str) -> &str {}
// i.e. fn longest(x: &str, y: &str) -> &str {}
// i.e. fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// 2) if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// i.e. fn first_word<'a>(s: &'a str) -> &'a str {}
// 3) if there are multiple input lifetime parameters, but one of them is &self or &mut self (method), the lifetime of self is assigned to all output lifetime parameters
fn first_word_pre<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn annouce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {}", annoucement);
        self.part
    }
}

// generic type parameters, trait bounds, and lifetimes together
use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annoucement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    {
        // static lifetime
        let s: &'static str = "I have a static lifetime.";
        println!("{}", s);

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:#?}", i);

        // let r;

        {
            let x = 5;
            let r = &x;
            println!("r: {}", r);
        }

        // println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// cannot because of a dangling reference to local variable reference
// fn longest_no<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// lifetime annotations
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime