fn main() {
    println!("Hello, world!");

    let capt_marvel = Film {
        title: String::from("Captain Marvel"),
        director: String::from("Anna Boden and Ryan Fleck"),
        studio: String::from("Marvel")
    };

    capt_marvel.describe();

    let elantris = Book {
        title: String::from("Elantris"),
        author: String::from("Brandon Sanderson"),
        publisher: String::from("Tor Books")
    };

    elantris.describe();    

    let let_it_be = Album {
        title: String::from("Let it be"),
        artist: String::from("Beattles"),
        label: String::from("Apple")
    };

    let_it_be.describe();   

    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat {lives: 9}),
        Box::new(Dog {name: String::from("Fido"), age: 5}),
    ];

    for pet in pets {
        println!("Hello, who are you {}", pet.talk());
    }

    let foo = String::from("Foo");
    let pair = duplicate(foo);
    println!("Pair - {:?}", pair);
}

struct Film {
    title: String,
    director: String,
    studio: String
}

struct Book {
    title: String,
    author: String,
    publisher: String
}

struct Album {
    title: String,
    artist: String,
    label: String
}

trait Catalog {
    fn describe(&self) {
        println!("We need more information for this type of media");
    }
}

impl Catalog for Film {
    fn describe(&self) {
        println!("{} was directed by {} through {} studios",
            self.title,
            self.director,
            self.studio);
    }
}

impl Catalog for Book {
    fn describe(&self) {
        println!("{} was written by {} and published by {}",
            self.title,
            self.author,
            self.publisher);
    }
}

impl Catalog for Album {
}

struct Dog { name: String, age: i8}
struct Cat { lives: i8}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String { format!("Woof, my name is {}!", self.name)}
}

impl Pet for Cat {
    fn talk(&self) -> String {String::from("Miau")}
}

fn duplicate<T: Clone>(a: T) -> (T,T) {
    (a.clone(), a.clone())
}