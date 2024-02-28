pub fn starting_point(){
//sample();
//not_working_demo();
//give_ownership();
//return_ownership();
}

 fn sample() {
    // We create an integer variable 'num1'.
    let num1 = 32;

    // 'num1' is copied into 'num2'. Unlike String, integers implement the 'Copy' trait.
    // This means 'num1' can still be used after this line.
    let num2 = num1;

    // This is perfectly valid because 'num1' was copied, not moved.
    println!("num1: {}", num1);

    // 'num2' is also valid and independent of 'num1'.
    println!("num2: {}", num2);

    // Passing 'num1' to a function by value.
    let num3 = add_one(num1);

    // 'num1' is still valid here because integers are Copy types.
    println!("num1 after function call: {}", num1);

    // 'num3' holds the result of the function.
    println!("num3: {}", num3);
}

// This function takes an integer and returns an integer.
fn add_one(x: i32) -> i32 {
    // Returns the input value incremented by 1.
    x + 1
} // 'x' goes out of scope here, but since it's a copy of the original, nothing special happens.

pub fn not_working_demo(){
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);
}

fn give_ownership() {
    // We create a String named 'text1'.
    let text1 = String::from("Hello");

    // 'text1' is moved into the function 'take_ownership'.
    // After this point, 'text1' is no longer valid in this scope.
    take_ownership(text1);
    //print!("{}",text1);



    // Uncommenting the next line will cause a compile error because 'text1' has been moved.
    // println!("After take_ownership: {}", text1);
}

// This function takes ownership of the String passed to it.
fn take_ownership(some_string: String) {
    // We can use 'some_string' inside this function.
    println!("Inside take_ownership: {}", some_string);
} // 'some_string' goes out of scope and 'drop' is called to free the memory.


fn return_ownership() {
    let text1 = String::from("Hello");

    // 'text1' is moved into 'give_and_take_back', but then it's returned and stored in 'text2'.
    let text2 = give_and_take_back(text1);

    // Now, 'text2' is valid and we can use it.
    println!("After give_and_take_back: {}", text2);

    // 'text1' is still invalid here.
}

// Takes ownership and then returns it.
fn give_and_take_back(a_string: String) -> String {
    // We can use 'a_string' here.
    println!("Inside give_and_take_back: {}", a_string);

    // 'a_string' is returned and its ownership is transferred back to the caller.
    a_string
}
