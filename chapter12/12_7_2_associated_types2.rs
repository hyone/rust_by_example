// the version that generize `first` and `last` method return types of `Contains` trait
// ( not hardcoded by `i32` )

use std::ops::Sub;

struct Container(i32, i32);

trait Contains {
    type A: Sub;
    fn contains(&self, &Self::A, &Self::A) -> bool;
    fn first(&self) -> Self::A;
    fn last(&self) -> Self::A;
}

impl Contains for Container {
    type A = i32;

    fn contains(&self, number1: &i32, number2: &i32) -> bool {
        (&self.0 == number1) && (&self.1 == number2)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> <C::A as Sub>::Output {
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
