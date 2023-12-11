fn main() {
    println!("Result of interporduct - {}", interproduct(20,30,40)); 
    println!("Result of fibonacci - {}", fibonacci(5));

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a*b + b*c + c*a;
}

fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        return n;
    } else {
        return fibonacci(n-2) + fibonacci(n-1);
    }
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0;3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }
    return result;
}