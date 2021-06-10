// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

static mut MY_STATIC_VARIABLES: i32 = 10;

pub fn run() {
  let name = "Aof";
  let mut age = 25;
  println!("My name is {} and I am {}", name, age);
  age = 30;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  unsafe {
    MY_STATIC_VARIABLES = 20;
    println!("MY_STATIC_VARIABLES: {}", MY_STATIC_VARIABLES);
  }

  // Assign multiple vars
  let ( my_name, my_age ) = ("Aof", 25);
  println!("{} is {}", my_name, my_age );
  
  // Casting
  let some_i32: i32 = 10;
  let some_i64: f64 = 20.2;
  let combined = some_i32 + some_i64 as i32;
  println!("combined: {}", combined);

  //Shadowing
  let var_a: i32 = 10;
  {
    println!("The Inner scope can see the outer var_a scope of {}", var_a);
    let var_a: String = "Aof".to_string();
    println!("But it can 'Shadow' it with it's own version of {}", var_a);
  }
  println!("See, var_a fro the outer scope is still {}", var_a);

}
