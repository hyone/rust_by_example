// Non-copyable types
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implment `DoubleDrop<T>` for any generic parameter `T`
// and caller `U`
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // //=> error: use of moved value: `empty`
    // null;
    // //=> error: use of moved value: `null`
}
