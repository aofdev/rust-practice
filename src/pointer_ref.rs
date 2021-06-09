pub fn run() {
  // Primitive Array
  let number1: [i32; 4] = [1, 2, 3, 4];
  let number2 = number1;
  println!("Number1 Values: {:?}", number1);
  println!("Number2 Values: {:?}", number2);

  // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

  // Vector
  let vec1: Vec<i32> = vec![1, 2, 3, 4];
  let vec2 = &vec1;

  println!("Vector Values: {:?}", (&vec1, vec2));
}
