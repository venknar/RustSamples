enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64},
}

enum Option<T> {
    Some(T),
    None,
}

enum Result {
    Ok(i32),
    Err(String)
}

fn main() {
    let quite: WebEvent = WebEvent::KeyPress('q');
    let n = 101;
    match divide_n_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
    
}

fn divide_n_two(n: i32) -> Result {
    if n %2 == 0 {
        Result::Ok(n/2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}
