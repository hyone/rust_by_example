struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    type A;
    type B;
    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are.
    // If the `input` type is `Container(i32, i32)`,
    // the `output` types are determined as `i32` and `i32`.
    type A = i32;
    type B = i32;

    fn contains(&self, number1: &i32, number2: &i32) -> bool {
        (&self.0 == number1) && (&self.1 == number2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number1 = 3;
    let number2 = 10;

    let container = Container(number1, number2);

    println!("Does container contain {} and {}: {}",
             &number1, &number2,
             container.contains(&number1, &number2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
