fn main() {
    let x: f32 = 9.0;
    let y: f32 = 2.0;
    println!("{}", add(x, y));
    println!("{}", sub(x, y));
    println!("{}", pro(x, y));
    println!("{}", quo(x, y));
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn sub<T: std::ops::Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

fn pro<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

fn quo<T: std::ops::Div<Output = T>>(x: T, y: T) -> T {
    x / y
}