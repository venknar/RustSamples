fn exclaim(input: String) {
    println!("{}", input);
}

fn last_char(input: String) -> char {
    if input.is_empty() {
        return '!';
    }   
    input.chars().next_back().unwrap()
}   

fn main() {
    exclaim("Hello, world!".to_string());
    let r : char = last_char(String::from("Hello, world!") );
    println!("{}", r);  
}

