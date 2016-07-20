// fn main() {
//     let mut optional = Some(0);
// 
//     loop {
//         match optional {
//             Some(i) => {
//                 if i > 9 {
//                     println!("Greater than 9, quit!");
//                     optional = None;
//                 } else {
//                     println!("`i` is `{:?}`. Try again.", i);
//                     optional = Some(i + 1);
//                 }
//             }
//         },
//         _ => { break; }
//     }
// }

fn main() {
    let mut optional = Some(0);
    // let mut optional: Option<i32> = None;

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
    // `if let` had additional optional `else`/`else if` clauses.
    // `while let` does not have these.
}
