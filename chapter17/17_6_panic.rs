struct Data {
    name: &'static str,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn main() {
    let _x = Box::new(0i32);
    let _y = Data { name: "y" };

    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point.
}
