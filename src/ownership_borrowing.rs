#[allow(unused_variables)]
pub fn run() {

 // STACK
 // - Fast memory creation and retrieval... speed!!!!
 // - Memory is automatically recaptured by the program after variables go out of scope
 // - Is the default in Rust
 // - Fixed size variables... collections cannot be stack based (and String are a collection of u8's)
 let stack_i8: i8 = 15;
 let stack_f32: f32 = 25.;
 let stack_bool: bool = true;
 let stack_char: char = 'a';

 // HEAP
 // - Flexibility
 // - Memory that can grow in size (Vector, HashMap, String, etc)
 // - Runtime performance cost (speed)
 // - Memory that can live beyond the scope that created it 
 // - Memory is automatically recaptured when the last OWNER goes out of scope
 let heap_vector: Vec<i8> = Vec::new();
 let heap_string: String = String::from("Aofdev");
 let heap_i8: Box<i8> = Box::new(30);

 let stack_i8_2 = stack_i8;
 println!("stack_i8: {}", stack_i8);
 println!("stack_i8_2: {}", stack_i8_2);

 let heap_i8_2 = heap_i8.clone(); // &heap_i8  <-- borrow with &
 println!("heap_i8: {}", heap_i8);
 println!("heap_i8_2: {}", heap_i8_2);

 let stack_f64: f64 = 120.;
 let heap_f64: Box<f64> = Box::new(25.);

 stack_procedure(stack_f64);
 println!("In function run stack {}", stack_f64);

//  heap_procedure(heap_f64); // "Ownership" of memory associated with heap_f64 gets transferred to "param"
//  println!("In function run heap {}", heap_f64) // heap_f64 no longer "owns" any memory

// Allocated memory needs to have one, and only one, owner
// One, and only one, owner of a piece of memory at a time
 heap_procedure(&heap_f64); 
 println!("In function run heap {}", heap_f64);

}



fn stack_procedure(mut param: f64) {
 param += 15.;
 println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) { // Borrow with &
  println!("In stack_procedure with param {}", param);
} // Memory automatically gets cleaned up here