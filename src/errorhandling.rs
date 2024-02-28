pub fn starting_point(){


 //  let result=factorial(11);
//print!("result ={}",result);
  
   //

//    let result = safe_divide(10.0, 2.0);
//     match result {
//         Ok(value) => println!("Result: {}", value),
//         Err(err) => println!("Error: {}", err),
//     }

    // let result = safe_divide_option(10.0, 2.0);
    // match result {
    //     Some(value) => println!("Result: {}", value),
    //     None => println!("Error: Cannot divide by zero"),
    // }
   unwrapexamples();

}

fn factorial(n: u64) -> u64 {
    //[profile.release]
//panic = "abort"

    if n > 10 {
        panic!("Input is too large, panicking!");
       
    } else if n == 0 {
        1
    } else {
        n * factorial(n - 1)
       
    }
}


fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn safe_divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn parse_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_default()
}

fn unwrapexamples() {
    let valid_number_str = "42";
    let invalid_number_str = "not_a_number";

    // Using unwrap_or
    let number1 = valid_number_str.parse::<i32>().unwrap_or(0);
    println!("Result using unwrap_or: {}", number1);

    // Using unwrap_or_default
    let number2 = parse_number(invalid_number_str);
    println!("Result using unwrap_or_default: {}", number2);

    // Using expect
    let number3 = valid_number_str.parse::<i32>().expect("Failed to parse");
    println!("Result using expect: {}", number3);

    // Using unwrap_or_else
    let number4 = invalid_number_str.parse::<i32>().unwrap_or_else(|_| {
        println!("Encountered an error, providing a fallback value.");
        -1
    });
    println!("Result using unwrap_or_else: {}", number4);
}
