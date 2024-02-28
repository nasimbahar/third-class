
pub fn starting_point(){
    //BasicExample();
   // enumwithdata();
    enumwithdifferntDataTypes();
}

enum IpAddrKind {
    V4,
    V6,
}

fn BasicExample() {
    // Create instances of each variant of 'IpAddrKind'.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Use these instances in a function.
    route(four);
    route(six);
}

// A function that takes any 'IpAddrKind'.
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Route for IPv4"),
        IpAddrKind::V6 => println!("Route for IPv6"),
    }
}


enum IpAddr {
    V4(String),
    V6(String),
}

fn enumwithdata() {
    // Create instances of each variant of 'IpAddr', with actual addresses.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Use these instances.
    display_address(home);
    display_address(loopback);
}

// A function to display the IP address.
fn display_address(ip: IpAddr) {
    match ip {
        IpAddr::V4(address) => println!("IPv4 address: {}", address),
        IpAddr::V6(address) => println!("IPv6 address: {}", address),
    }
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Implement different behaviors based on the variant
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn enumwithdifferntDataTypes() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    let m: Message=Message::ChangeColor(3, 4, 9);
    m.call();
}
