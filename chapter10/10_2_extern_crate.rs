// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_function();

    // rary::private_function();
    // //=> error: function `private_function` is private

    rary::indirect_access();
}
