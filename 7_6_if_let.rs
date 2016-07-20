fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let` construct
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    //  if you need to specify a failre, use an `else` clause.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    };

    // Provide an altered failing condition.
    let i_like_letters = false;

    // and also we can use `else if`
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
