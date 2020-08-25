// variables are immutable by default
// if you want to change the var, need add "mux" keyword
pub fn run() {
    let mut name = "CloudHuang";

    println!("My Name is {}", name);

    let age = 38;
    name = "Liping Huang";

    println!("My Name is {} and I am {}", name, age);

    const PI: f64 = 3.1415;
    println!("PI: {}", PI);

    let (my_name, my_ag) = ("CloudHuang", 38);
    println!("{} is {}", my_name, my_ag);
}
