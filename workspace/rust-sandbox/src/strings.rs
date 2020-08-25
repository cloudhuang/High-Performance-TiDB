pub fn run() {
    let hello = "hello world";
    let world = String::from("Hello World");

    println!("{:?}", (hello, world));

    // get length
    println!("{:?}", hello.len());

    // push char
    // push string
    let mut say_hi = String::new();
    say_hi.push('H');
    say_hi.push_str("ello World");
    println!("{:?}", say_hi);

    let hello: &'static str = "Hello, world!";
    println!("{:?}", hello);

}
