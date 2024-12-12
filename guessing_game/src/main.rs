fn main() {
    // integers

    let a: u8 = 2;                                // numbers from 0 - 255
    let b: i8 = -34;                              // numbers from -128 - 127
    let c: u16 = 1000;                            // numbers from 0 - 65535
    let d: i16 = -20000;                          // numbers from -32768 - 32767
    let e: u32 = 1000000;                         // numbers from 0 - 4294967295
    let f: i32 = -5000000;                        // numbers from -2147483648 - 2147483647
    let g: u64 = 10000000000;                     // numbers from 0 - 18446744073709551615
    let h: i64 = -80000000000;                    // numbers from -9223372036854775808 - 9223372036854775807
    let i: u128 = 1000000000000000000000000;      // numbers from 0 - 340282366920938463463374607431768211455
    let j: i128 = -9000000000000000000000000;     // numbers from -170141183460469231731687303715884105728 - 170141183460469231731687303715884105727

    // floating-points
    let x: f32 = 3.14;                            // 32-bit floating-point
    let y: f64 = 2.71828182845;                   // 64-bit floating-point

    // boolean types
    let is_true: bool = true;                     // Boolean type (true)
    let is_false: bool = false;                   // Boolean type (false)

    // single character type
    let ch: char = 'A';                           // Character type

    // strings
    let str1: &str = "Hello, world!";             // String slice
    let str2: String = String::from("Hello");     // String object

    println!("u8: {}", a);
    println!("i8: {}", b);
    println!("u16: {}", c);
    println!("i16: {}", d);
    println!("u32: {}", e);
    println!("i32: {}", f);
    println!("u64: {}", g);
    println!("i64: {}", h);
    println!("u128: {}", i);
    println!("i128: {}", j);

    println!("f32: {}", x);
    println!("f64: {}", y);

    println!("bool (true): {}", is_true);
    println!("bool (false): {}", is_false);

    println!("char: {}", ch);

    println!("&str: {}", str1);
    println!("String: {}", str2);
