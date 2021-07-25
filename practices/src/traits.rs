use std::iter;
use std::vec::IntoIter;

// Derive
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

// Returning Traits with dyn
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: i16) -> Box<dyn Animal> {
    if random_number < 200 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// Iterators

struct NextNumber {
    curr: i8,
    next: i8,
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for NextNumber {
    // We can refer to this type using Self::Item
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

fn next_number() -> NextNumber {
    NextNumber { curr: 0, next: 1 }
}

// impl Trait
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

// Disambiguating overlapping traits
trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn run() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // println!("One second looks like: {:?}", _one_second);

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(15);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(20.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);

    let random_number = 300;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the next_number sequence are: ");
    for i in next_number().take(6) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the next_number sequence are: ");
    for i in next_number().skip(6).take(2) {
        println!("> {}", i);
    }

    // impl Trait
    let numbers = vec![1, 2, 3, 4, 5];
    double_positives(&numbers).for_each(|x| println!("{}", x));

    // Disambiguating overlapping traits
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
