use std::fmt::Debug;

pub fn run() {
    run_generic_lifetimes();
    run_generic_multiple_lifetimes();
    run_lifetime_struct();
    run_method_lifetime();
    run_generic_lifetime_parameters_trait_bounds();
}

// Generic Lifetimes
fn run_generic_lifetimes() {
    let string1 = String::from("hello");
    {
        let string2 = String::from("xyz");
        let result = generic_lifetimes(string1.as_str(), string2.as_str());
        println!("The run_generic_lifetimes string is {}", result);
    }
}

fn generic_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic Multiple Lifetimes
fn run_generic_multiple_lifetimes() {
    let string1 = String::from("hello");
    let string2 = String::from("word");
    let result = generic_multiple_lifetimes(string1.as_str(), string2.as_str());
    println!("The run_generic_multiple_lifetimes string is {}", result);
}
fn generic_multiple_lifetimes<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

//Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn run_lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is {}", i.part);
}

// Methods Lifetime
struct Owner(i32);
impl Owner {
    fn add_two<'a>(&'a mut self) {
        self.0 += 2;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn run_method_lifetime() {
    let mut owner = Owner(5);
    owner.add_two();
    owner.print();
}

// Generic Lifetime Parameters, Trait Bounds
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn run_generic_lifetime_parameters_trait_bounds() {
    let x = 8;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
