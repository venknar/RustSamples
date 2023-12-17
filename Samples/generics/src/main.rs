fn main() {
   
    println!("Pick a number: {:?}", pick(98, 222, 333));
}

fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n %2 == 0 {
        even
    } else {
        odd
    }
}