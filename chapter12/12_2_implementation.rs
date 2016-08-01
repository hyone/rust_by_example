struct Val (f64,);
struct GenVal<T>(T,);

impl Val {
    fn value(&self) -> &f64 { &self.0 }
}

// These commented codes and `impl <T> GenVal<T>` block are exclusive.
// because the existance of multiple applicable implementations is prohibited.

// impl GenVal<i32> {
//     fn value(&self) -> &i32 {
//         println!("This is a method of GenVal<i32>");
//         &self.0
//     }
// }
// 
// impl GenVal<bool> {
//     fn value(&self) -> &bool {
//         println!("This is a method of GenVal<bool>");
//         &self.0
//     }
// }

impl <T> GenVal<T> {
    fn value(&self) -> &T {
        println!("This is a method of GenVal<T>");
        &self.0
    }
}

fn main() {
    let x = Val(3.0);
    let y = GenVal(3132);
    let z = GenVal(true);

    println!("{}, {}, {}", x.value(), y.value(), z.value());
}
