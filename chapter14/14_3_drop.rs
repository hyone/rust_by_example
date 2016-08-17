#[derive(Debug)]
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just after exited block B");

        println!("Exiting block A");
    }
    println!("Just after exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);

    // // This causes error. becaue `_a` is manually droped.
    // println!("{:?}", _a);
    // //=> error: use of moved value: `_a` [E0382]

    println!("end of the main function");

    // `_a` *won't* be *drop*ed again here,
    // because it already has been manually *drop*ed
}
