pub fn run() {
    meeting("Hello", "Aof");

    // Bind function values to variables
    let get_sum = add(15, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(5, 3));

    //Store a function in a variable.
    let fn_variable = add;
    println!("calling using function variable {}", fn_variable(10, 20));

    // Higher Order function
    let result = higer_order_fn(10, add_one);
    println!("calling higher order function {}", result);

    // Anonymous functions
    let anonymous_result = higer_order_fn(24, |x: i32| x + 1);
    println!("calling higher order function with {}", anonymous_result);

    // Higher Order function - Return a function from function
    let step_value = &10;
    let step_function = higer_order_fn_return(step_value);
    println!("the stepped value is {}", step_function(50));
}

fn meeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn higer_order_fn<F>(value: i32, step: F) -> i32
where
    F: Fn(i32) -> i32,
{
    step(value)
}
fn add_one(x: i32) -> i32 {
    x + 1
}

fn higer_order_fn_return<'a>(step_value: &'a i32) -> Box<dyn Fn(i32) -> i32 + 'a> {
    Box::new(move |x: i32| x + step_value)
}
