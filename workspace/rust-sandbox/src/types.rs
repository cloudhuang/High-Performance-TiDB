/*
Primitive Types:
Integers: u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
Floats: f32 f64
Boolean: bool
Characters: chat
Tuples
Arrays
*/

pub fn run() {
    // default type is "i32"
    let x = 100;
    // default type is "f64"
    let y = 3.1415;

    // Boolean
    let z = true;

    // get bool from expression
    let is_greater = 10 > 5;
    println!("10 is greater than 5:  {:?}", is_greater);

    // char
    let a = 'a';
    println!("{:?}", a);

    let face = '\u{1F600}';
    println!("face - {:?}", face);

    println!("{:?}", (x, y, z));

    let x: i64 = 5434567854345678;
    println!("{:?}", (x, y, z));

    println!("Max i32: {}", std::i32::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Min i32: {}", std::i32::MIN);

    println!("Max i64: {}", std::i64::MAX);
    println!("Min i64: {}", std::i64::MIN);
}
