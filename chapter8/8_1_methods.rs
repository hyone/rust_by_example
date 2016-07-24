struct Point {
    x: f64,
    y: f64,
}

// Implementation block,
// all `Point` methods go in here
impl Point {
    // This is a static method.
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Self::new(0.0, 0.0)
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method.
    // `&self` is sugar for `self: &Self`,
    // where `Self` is the type of the caller object.
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Calling instance method.
    // note that the first argument `&self` is implicitly passed
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // // Error!
    // // `rectangle` is immutable, but this method requires a mutable one.
    // rectangle.translate(1.0, 0.0);

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // // Error!
    // // previous `destroy` call "consumed" `pair`
    // pair.destroy();
}
