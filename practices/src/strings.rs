// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
#[allow(unused_variables)]

pub fn run() {
    //2 kinds of strings
    let example_slice: &str = "Howdy"; //mostly immutable
    let example_string: String = String::from("Partner");
    //both are groups o char
    //String is on the heap and mutable
    //&str is immutable, can be kept on stack, heap, or embedded
    //str slice is great for runtime speed

    //let text = "first" + "second"; //this will fail
    let combine_string_literals = ["first", "second"].concat(); //bumps it to a String
    let combine_with_macro = format!("{} {}", "first", "second");

    // will make sense once you know borrowing
    let mut mut_string = String::new();

    mut_string.push_str("some hardcoaded literal");
    mut_string.push('m');
    //push is meant for char, push_str is or string slices, char is not a single character string
    //char has a lot more info than a string

    let a = String::from("a");
    let b = String::from("b");
    let bombined = a + &b + &mut_string;

    let string_from_substring: &str = &bombined[..=2];
    //this includes the char at index 2

    let char_by_index = &example_string.chars().nth(1); //can't retrieve a char from a slice of a single letter
                                                        //nth brings back an "option" a safe type that requries a handling
                                                        //handling at compile time prevents crashing
                                                        //same on both String and &str

    match char_by_index {
        Some(c) => println!("found a char {}", c),
        None => {} //empty block
    }

    if let Some(c) = example_string.chars().nth(2) {
        println!("found a char {}", c);
    }

    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
