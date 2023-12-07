fn main() {
    println!("Hello, world!");
    let mut x: i32 = 1;
    println!("The value of x is : {}", x);

    x = 2;
    println!("The value of x is : {}", x);

    let y: bool = true;
    println!("The value of y is : {}", y);

    let y: bool = false;
    println!("The value of y is : {}", y);

    const STRING: &str = "Hello";
    println!("The value of string constant is : {}", STRING);

    let weight: f64 = 100.2;
    println!("The Value of weight is : {}", weight);
}
