fn main() {
  let say: String = String::from("Hello World!");
  print_out(say.clone()); // clone is deep copy and take ownership of the string.
  print_out_borrow(&say); // Example of the borrow concept. 
  println!("Again: {}", say);

  let mut my_vec = vec![1,2,3];
  println!("{:?}", my_vec);

  add_to_vec(&mut my_vec);
  println!("{:?}", my_vec);
}

fn print_out(to_print: String) {
    println!("{}", to_print);
}

fn print_out_borrow(to_print: &String) {
    println!("{}", to_print);
}

fn add_to_vec(a_vec: &mut Vec<i32>) {
    a_vec.push(4);
}

