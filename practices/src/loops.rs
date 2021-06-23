pub fn run() {
    // let mut count = 0;

    // Infinite Loop
    // loop {
    //   count += 1;
    //   println!("Number: {}", count);

    //   if count == 30 {
    //     break;
    //   }
    // }

    // While Loop
    // while count <= 100 {
    //   if count % 15 == 0 {
    //     println!("fizzbuzz");
    //   } else if count % 3 == 0 {
    //     println!("fizz");
    //   } else {
    //     println!("{}", count);
    //   }

    //   count += 1;
    // }

    // For Range from 0 to 204
    for i in 0..204 {
        if i % 15 == 0 {
            println!("Jump");
        } else if i % 202 == 0 {
            println!("bomb")
        } else {
            println!("{}", i);
        }
    }
}
