struct Container(i32, i32);

trait Contains<A, B> {
    // explicitly requires `A` and `B`
    fn contains(&self, &A, &B) -> bool;

    // doesn't explicitly requires `A` or `B`
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number1: &i32, number2: &i32) -> bool {
        (&self.0 == number1) && (&self.1 == number2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`.
// In light of that, having to express `A` and `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
        container.last() - container.first()
    }
// fn difference<A, B>(container: &Contains<A, B>) -> i32 {
    // container.last() - container.first()
// }

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
