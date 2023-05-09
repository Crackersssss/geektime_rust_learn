fn main() {
    let mut arr = vec![1, 2, 3];
    let last = arr.last();
    println!("last: {:?}", last.unwrap());
    arr.push(4);
    println!("arr: {:?}", arr);
}