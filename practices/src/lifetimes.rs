pub fn run() {
    let string1 = String::from("hello");
    let string2 = "word";

    let result = welcome(string1.as_str(), string2);
    println!("The welcome string is {}", result);
}

fn welcome<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
