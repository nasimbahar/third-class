pub fn starting_point(){
    main_example();
    copytype_example();
    mutable_copy_example();
}

fn main_example() {
    // We create a String.
    let my_string = String::from("Hello, Rust!");

    // We borrow 'my_string' by creating a reference to it. 
    // Note: This is an immutable borrow.
    let string_length = calculate_length(&my_string);

    // We can still use 'my_string' here because we've only borrowed it, not moved it.
    println!("The length of '{}' is {}.", my_string, string_length);

    // Now we create a mutable reference to 'my_string'.
    // Important: There can only be one mutable reference to a piece of data in a particular scope.
    let mut my_mut_string = String::from("Hello, Rust!");
    append_exclamation(&mut my_mut_string);

    // We can still use 'my_mut_string' after it's been mutably borrowed.
    println!("After modification: {}", my_mut_string);
}

// This function takes an immutable reference to a String.
fn calculate_length(s: &String) -> usize {
    // It can read 's', but not modify it.
    s.len()
} // 's' goes out of scope here, but since it does not own what it refers to, nothing happens.

// This function takes a mutable reference to a String.
fn append_exclamation(s: &mut String) {
    // It can modify 's'.
    s.push_str("!");
} // 's' goes out of scope here, but since it does not own what it refers to, nothing happens.


fn copytype_example() {
    let num = 10; // 'num' is an integer, which is a 'Copy' type.

    // Borrow 'num' by creating an immutable reference.
    let num_borrowed = &num;

    // 'num' can still be used directly because it's a 'Copy' type.
    println!("Original value: {}", num);

    // 'num_borrowed' is a reference to 'num'.
    println!("Borrowed value: {}", num_borrowed);

    // Call a function with a reference to 'num'.
    let doubled = double(&num);

    // 'num' is still valid here.
    println!("Value after function call: {}", num);
    println!("Doubled value: {}", doubled);
}

fn double(x: &i32) -> i32 {
    // Dereference 'x' and double the value.
    *x * 2
}


fn mutable_copy_example() {
    let mut num = 10; // 'num' is an integer, which is a 'Copy' type.

    // We create a mutable reference to 'num'.
    let num_ref = &mut num;

    // Modify 'num' through its mutable reference.
    *num_ref += 5;

    // We can't use 'num' directly here while it's mutably borrowed.
    // Uncommenting the next line will cause a compile-time error.
    // println!("Original value: {}", num);

    // 'num_ref' is still valid and reflects the updated value.
    println!("Updated value through reference: {}", num_ref);

    // The mutable borrow ends here, so 'num' can be used again.
    // 'num' now reflects the updated value.
    println!("Value of num after borrow ends: {}", num);
}

