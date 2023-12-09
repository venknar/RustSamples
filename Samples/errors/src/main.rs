fn main() {

    println!("Hello, world!");

    let fruits: Vec<&str> = vec!["banana", "apple", "coconut"];

    let first: Option<&&str> = fruits.get(0);
    println!("{:?}", first);

    let third: Option<&&str> = fruits.get(2);
    println!("{:?}", third);

    let non_existent: Option<&&str> = fruits.get(5);
    println!("{:?}", non_existent);

    //let v: Vec<i32> = vec![0, 1, 2, 3];
   // println!("{}", v[6]);
}
