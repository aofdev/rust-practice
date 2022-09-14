use std::fmt::Debug;

fn logger_impl(t: impl Debug) -> () {
    println!("{:?}", t)
}

fn logger_generic<T: Debug>(t: T) -> () {
    println!("{:?}", t)
}

fn logger_dyn(t: Box<dyn Debug>) -> () {
    println!("{:?}", t)
}

fn logger(ts: Vec<Box<dyn Debug>>) -> () {
    ts.into_iter().for_each(|t| println!("{:?}", t));
}

#[derive(Debug)]
pub struct A {
    pub a: i32,
}

fn main() {
    let a = A { a: 1 };
    logger_impl("test");
    logger_generic(123);
    logger_dyn(Box::new(true));
    logger(vec![
        Box::new(false),
        Box::new(20),
        Box::new(1.10),
        Box::new(a),
        Box::new("rust".to_string()),
    ])
}
