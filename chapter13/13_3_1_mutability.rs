#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutable_book = Book {
        author: "Douglas  Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979
    };

    // Create a mutable copy of `immutable_book` and call it `mutable_book`
    let mut mutable_book = immutable_book;

    // borrow an immutable object as immutable
    borrow_book(&immutable_book);

    // borrow a mutable object as immutable
    borrow_book(&mutable_book);

    // borrow a mutable object as mutable
    new_edition(&mut mutable_book);

    // new_edition(&mut mutable_book);
    // //=> error: mismatched types [E0308]
    // //          note: expected type `&mut Book`
    // //          note:    found type `&Book`

    // // borrow a immutable object as mutable
    // new_edition(&mut immutable_book);
    // //=> error: cannot borrow immutable local variable `immutable_book` as mutable
}
