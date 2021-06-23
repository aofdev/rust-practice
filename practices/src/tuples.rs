// Tuples group together values of different types
// Max 12 elements
#[allow(unused_variables)]
pub fn run() {
    let person: (&str, &str, i8) = ("John", "Dave", 30);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.1, 2.2));
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My full tuple is {:?}", some_tuple);

    //nested tuple, compiler is a bit weird need a random space
    let some_val = (some_tuple.5).1;

    fn get_some_rgb() -> (u8, u8, u8) {
        (255, 10, 2) // return
    }

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);

    let (my_red, my_green, my_blue) = some_color;
    println!("r {} g {}, b {}", my_red, my_green, my_blue);

    //unit tuple
    let empty_tuple = ();
    //used in match statements for doing nothing at the end of a branch
    //procedures are actually functions that return an empty tuple
}
