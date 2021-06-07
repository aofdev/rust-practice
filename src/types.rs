/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
#[allow(unused_variables)]

pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 4545445454545;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  //integers
  let an_integer: i8 = 100; //2^8, 256 whole numbers -128 to +127 (0 included)

  println!("min i8 is {}", std::i8::MIN);
  println!("max i8 is {}", std::i8::MAX);

  let overflow_test = an_integer + 120; //220 greater than datatype allows
  println!("{}", overflow_test); //this should PANIC, which is a kind of crash

  let a_pos_integer: u8 = 10; //from 0 to 255 //u stand for unsigned
  //used quite often, for example for color values

  let big_data: i128 = 10;
  //i32 is in the +- 2billion range, i64 is really big 
  //i32 is the default

  let some_isize: isize = 10; //depends on if the computer is 32 or 64 bit
  let some_usize: usize = 10;

  //float
  //only 2 kinds, not a perfect representation
  //can only keep so much precision
  //communicate the compiler you need a .
  let a_float: f32 = 10.; 
  let another_float: f64 = 10.;
  //assumes f64 by default

  // Boolean
  let is_active: bool = true;
  let implied_data = false; //Rust assumes you want a boolean


  // Get boolean from expression
  let is_greater: bool = 10 < 5;
  
  //char
  let some_char: char = 'a';
  //is 4 bytes, but holds much more than ascii
  //handles chinese, emoji, etc.
  //not a string, don't confuse them.
  let a1 = 'a';
  let face = '\u{1F600}';

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
