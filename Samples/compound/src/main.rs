fn main() {
    println!("Hello, world!");

    let array: [u32; 5] = [1u32, 2, 3, 4, 5];
    let tuple: (u32, i32, bool) = (1u32, 2, true);
    let first_element = tuple.0;
    let element = tuple.1;
}
