pub fn run() {
    // print to console
    println!("{}", "hello world from print.rs file");

    // print number
    println!("Number: {}", 1);

    // basic formatting
    println!("Name: {}, Age: {}", "CloudHuang", 37);

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "CloudHuang", "Mars", "code"
    );

    // named arguments
    println!(
        "{name} likes {activity}",
        name = "CloudHuang",
        activity = "Code"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (100, true, "Hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
