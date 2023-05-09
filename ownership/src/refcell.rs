use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);
    {
        let mut data = data.borrow_mut();
        *data += 1;
    }
    println!("data: {:?}", data)
}