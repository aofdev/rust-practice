macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

// A simplified version of the vec! macro definition
macro_rules! vec_sample {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[allow(clippy::eq_op, clippy::vec_init_then_push)]
pub fn run() {
    print_result!(20 + 30 + 50);
    test!(2 == 2; and 1 == 1);
    test!(true; or false);
    let v: Vec<u32> = vec_sample![1, 2, 3];
    // let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", v)
}
