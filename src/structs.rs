// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

#[derive(Debug)]
struct KylesData {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    // rust embraces composition over inheritence
    // instead of extending you include
    random: RandomInfo,
}

#[derive(Debug)]
struct RandomInfo {
  pub some_int: i8, //rust will assume everything is private unless made public
  pub some_float: f32,
  pub call_count: i64,
}

impl RandomInfo {
  fn new(param_a: i8) -> Self { //capital Self is the type of self
      Self {
          some_int: param_a,
          some_float: 9.0,
          call_count: 0,
      }
  }

  fn is_smaller(&mut self, compare_to: i8) -> bool { //lowercase self is the struct itself, with the & borrowing memory from the heap
      self.call_count += 1;
      
      self.some_int < compare_to
  }
}

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  c.green = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut p = Person::new("John", "Dave");
  println!("Person {}", p.full_name());
  p.set_last_name("Williams");
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());


  let mut kyles_var = KylesData {
      some_bool: true,
      some_int: 80,
      some_float: 10.3,
      random: RandomInfo {
          some_int: 8,
          some_float: 8.,
          call_count: 0,
      },
  };

  kyles_var.some_int = 100;

  let dougs_var_2 = KylesData {
      some_int: 200, //manually set things before spreading any inherited values
      ..kyles_var
  };
  println!("dougs_var_2: {:?}", dougs_var_2);

  let mut random_info_var = RandomInfo {
      some_int: 4,
      some_float: 3.,
      call_count: 0,
  };

  let kyles_var = KylesData {
      some_bool: true,
      some_int: 80,
      some_float: 10.3,
      random: RandomInfo::new(3), //2 colons, class method
  };
  println!("kyles_var: {:?}", kyles_var);
  
  let is_this_smaller = random_info_var.is_smaller(9); //dot operator if using self, instance method, use . 
  println!("is_this_smaller: {:?}", is_this_smaller);
}
