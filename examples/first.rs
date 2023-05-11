fn main() {
    let x = 1;
    let y = 2;
    println!("{}", add(x, y));
    let x = 3.2;
    let y = 1.2;
    println!("{}", sub(x, y));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

