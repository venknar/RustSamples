fn main() {
    println!("Hello, world!");

    let a: u8 = 5;
    let b: u8 = 1; 

    let x = 2.0; // f64 
    let y: f32 = 3.0; // f32    

    let sum = a + b;
    let difference = x - 1.0;
    let product = 4 * a;
    let quotient = 9.0/y;

    let remainder = a % b;

    let t = true;
    let f: bool = false;    

    let _c: char = 'z';  
    let _d: char = 'a';

    let array = [1, 2, 3, 4, 5];
    println!("The value of array is : {}", array[1]);

    let mytuple = (true, 1, 2);
    println!("The value of mytuple is : {}", mytuple.1);
}
