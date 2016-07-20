use std::fmt::{ Display, Formatter, Result };

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f64);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

// use struct as types
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rectangle [({}, {}), ({}, {})]",
               self.p1.x, self.p1.y,
               self.p2.x, self.p2.y)
    }
}

fn rect_area(&Rectangle {
    p1: Point { x: left, y: bottom },
    p2: Point { x: right, y: top },
}: &Rectangle) -> f64 {
    ((right - left) * (top - bottom)).abs()
}

fn square(point: Point, _size: f32) -> Rectangle {
    let Point { x: left, y: bottom } = point;
    let size = _size as f64;
    Rectangle {
        p1: point,
        p2: Point { x: left + size, y: bottom + size }
    }
}

fn main() {
    let point: Point = Point { x: 0.3, y: 0.4 };

    // access fields of struct
    println!("point coordinates: ({}, {})", point.x, point.y);

    // destructing
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // destructing
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle = Rectangle {
        p1: Point { x: 2.0, y: 2.0 },
        p2: Point { x: 5.0, y: 6.0 }
    };
    println!("rect_area = {:?}", rect_area(&rectangle));

    let square: Rectangle = square(Point { x: 2.0, y: 3.0 }, 5.0);
    println!("square = {}, it's size = {:?}", square, rect_area(&square));
}
