pub fn run() {
  let age: u8 = 22;
  let check_id: bool = true;
  let knows_person_of_age = true;

  // If/Else
  if age >= 21 && check_id || knows_person_of_age {
    println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry, you have to leave");
  } else {
    println!("Bartender: I'll need to see your ID");
  }

  // Shorthand If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age: {}", is_of_age);

  let some_bool = true;
  let some_int = 55;

  let var_from_inline_multiple_conditional = if some_int == 9 {
      300 
  } else if some_int == 10 {
      println!("can put other lines here");
      100 
  } else { 
      400 
  };
  println!("var_from_inline_multiple_conditional {}", var_from_inline_multiple_conditional);

  match some_int { 
      0 => println!("hit 0 branch"),
      1..=100 => {
          println!("Between 1 and 100 branch");
          println!("more stuff");
      },
      _ => println!("Else branch"),
  }

  let var_from_match_inline = match some_bool {true => 10, false => 20}; 
  let var_from_match_multi_line = match some_int {
      0 => 0,
      1 | 2 => 100,
      _ => 200,
  };
  
  println!("var_from_match_inline: {}", var_from_match_inline);
  println!("var_from_match_multi_line: {}", var_from_match_multi_line);
}
