fn main() {
    println!("Hello, world!");

    let v = vec![1u8, 2, 3];
    println!("{}", v.len());

    use std::collections::HashMap;
    let mut enrollment = HashMap::new();
    enrollment.insert("Biology", "11");
    enrollment.insert("Physics", "22");
    enrollment.insert("Chemistry", "33");

    println!("Hash map result - {}", enrollment.get("Biology").unwrap());
}
