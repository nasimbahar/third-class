pub fn starting_point(){
    //sampleExample();
   complexexample();
}

// Define a struct named 'User'.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn sampleExample() {
    // Create an instance of the 'User' struct.
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };

    // Access the fields of the struct.
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
}



struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are defined within an 'impl' (implementation) block.
impl Rectangle {
    // A method to calculate the area of a Rectangle instance.
    // Methods take a self parameter to access the instance they're called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // An associated function (does not take self as a parameter).
    // It's often used for constructors.
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn complexexample() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // Call the method 'area' on our instance.
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // Call the associated function 'square'.
    let square = Rectangle::square(10);
    println!("Square size: {}x{}", square.width, square.height);
}
