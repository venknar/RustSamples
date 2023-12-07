struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name:String::from("Adam"),
        age: 25,
        likes_oranges: true
    };

    let origin = Point2D(100, 200);

    println!("Person named is : {}", person.name);
    println!("Person age is : {}", person.age);
    println!("Hello, world!");

    println!("Point origin is {}", origin.0);
}
