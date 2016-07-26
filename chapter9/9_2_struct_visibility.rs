mod my {
    // A public struct with a public field
    pub struct WhiteBox<T> {
        pub contents: T
    }

    // A public struct with a private field
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents
            }
        }
    }
}

fn main() {
    // public structs with public fields can be constructed as usual
    let white_box = my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The white box contains: {}", white_box.contents);

    // public structs with private fields can not be constructed by using field names.

    // let black_box = my::BlackBox { contents: "classified information" };
    // //=> error: field `contents` of struct `my::BlackBox` is private

    let _black_box = my::BlackBox::new("classified information");

    // println!("The black box contains: {}", _black_box.contents);
    // //=> error: field `contents` of struct `my::BlackBox` is private
}
