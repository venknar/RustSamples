fn main() {
    println!("Hello, world!");

    let s: String = String::from("Hi\nBye");
    println!("{}", s);

    let l: &str = s.lines().next().unwrap();
    println!("{}", l);

    let text: &str = "Hello\nWorld\n";
    println!("{}", new_first_line(text));
}

pub fn first_line(string: String) -> String {
   return string.lines().next().unwrap().to_owned();
}


pub fn new_first_line(string: &str) -> &str {
    return string.lines().next().unwrap();
 }