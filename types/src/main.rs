fn division_marks() {
    println!("{}", "=".repeat(20));
}

fn main() {
    // types in rust 
    let x: i32 = 42;
    let y: f64 = 3.14;
    let z: bool = true;
    let a: char = 'a';
    let b: String = "hello".to_string();
    let c: &str = "world";
    let d: Vec<i32> = vec![1, 2, 3, 4, 5];
    let e: (i32, f64) = (10, 20.5);

    let decimal = 42.0; 
    let hex = 0x2A; 
    let octal = 0o52; 
    let binary = 0b101010;  
    let byte = b'A'; 

    let tup: (i32, f64, u8) = (10, 20.5, 1); 
    let (f, g, h) = tup;

    let first_tup = tup.0; 
    let second_tup = tup.1;
    let third_tup = tup.2;

    println!("The value of f is: {}", f);
    println!("The value of g is: {}", g);
    println!("The value of h is: {}", h);  
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    println!("The value of d is: {:?}", d);
    println!("The value of e is: {:?}", e);
    division_marks();

    println!("The value of decimal is: {}", decimal);
    println!("The value of hex is: {}", hex);
    println!("The value of octal is: {}", octal);
    println!("The value of binary is: {:b}", binary);
    println!("The value of byte is: {}", byte);
    division_marks();

    println!("The value of tup is: {:?}", tup);
    println!("The value of first_tup is: {}", first_tup);
    println!("The value of second_tup is: {}", second_tup);
    println!("The value of third_tup is: {}", third_tup);

     


}