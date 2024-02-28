pub fn starting_point(){

    //enum Option<T> {
     //   Some(T),
      //  None,
      //  }
    BasicExample();
   
}

fn BasicExample() {
    // An `Option` holding an integer.
    let some_number = Some(42);
    // An `Option` holding a string.
    let some_string = Some("a string");

    // An `Option` that holds nothing.
    let absent_number: Option<i32> = None;

    // Use these options in a function.
    print_option(some_number);
    print_option(some_string);
    print_option(absent_number);
}

// A function that takes an `Option` and prints its content.
fn print_option<T>(option: Option<T>) {
    match option {
        Some(_) => println!("Some"),
        None => println!("None"),
    }
}
