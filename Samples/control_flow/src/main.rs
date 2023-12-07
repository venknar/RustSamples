fn main() {
    if 1 == 2 {
        println!("math is broken");
    } else {
        println!("everything is fine");
    }

    let formal: bool = true;
    let _greeting: () = if formal {
        println!("Good evening");
    } else {
        println!("Hey Friend");
    };

    let number: i32 = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 3 or 4");
    }

    let boolean: bool = true;
    let _binary = match boolean {
        false => 0,
        true => 1
    };


}
