use std::fmt;

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

#[link(name = "m")]
extern {
    // This is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;
}

// Since calling foreign function is considered to be unsafe,
// it's common to write safe wrappers around them.
fn sqrt(z: Complex) -> Complex {
    unsafe { csqrtf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 0. };
    let z_sqrt = sqrt(z);

    println!("the square root of {:?} is {:?}", z, z_sqrt);
}
