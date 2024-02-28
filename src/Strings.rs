pub fn starting_point(){

   //BasicExample();

   let input = "hello, world";
     let reversed = reverse_string_third(input);
     println!("Original: {}", input);
     println!("Reversed: {}", reversed);
    // let reversed = reverse_string(input);
    // println!("Original: {}", input);
    // println!("Reversed: {}", reversed);
}

fn BasicExample() {
    // String literal: immutable and stored in the binary
    let hello_literal: &str = "Hello, world!";

    // String object: mutable and stored on the heap
    let mut hello_string: String = String::from("Hello");
    
    // Appending to a String object
    hello_string.push_str(", world!");

    println!("String Literal: {}", hello_literal);
    println!("String Object: {}", hello_string);
}




fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn reverse_string_third(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect(); 
    let mut reversed = String::with_capacity(input.len());
    
    // Manually iterating from the last character to the first
    let mut index = chars.len();
    while index > 0 {
        index -= 1; // Decrement first to get the last character's index
        reversed.push(chars[index]);
    }
    
    reversed
}