#![allow(dead_code, clippy::upper_case_acronyms)]

// This stays the same
#[derive(Debug)]
enum Colour {
    RGB { r: u8, g: u8, b: u8 },
    Grayscale(u8),
}

trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f32;
}

struct Square {
    side: u8,
}

struct Circle {
    radius: f32,
}

struct Rectangle(u8, u8);

// {{{
impl Shape for Square {
    fn name(&self) -> &str {
        "square"
    }

    fn area(&self) -> f32 {
        (self.side * self.side) as f32
    }
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "circle"
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &str {
        "rectangle"
    }

    fn area(&self) -> f32 {
        self.0 as f32 * self.1 as f32
    }
}
// }}}

// NOTE: SUDDEN GENERICS APPEAR ‼️
struct ColouredShape<S>
where
    S: Shape,
{
    colour: Colour,
    shape: S,
}

// This stays the same
impl<S> ColouredShape<S>
where
    S: Shape,
{
    fn describe(self) {
        println!(
            "This shape is a {:?} {} and has an area of {:?}",
            self.colour,
            self.shape.name(),
            self.shape.area(),
        );
    }
}

pub fn main() {
    // {{{
    struct Point;
    impl Shape for Point {
        fn name(&self) -> &str {
            "point"
        }

        fn area(&self) -> f32 {
            0f32
        }
    }
    // }}}

    let shnep = ColouredShape {
        colour: Colour::Grayscale(123),
        shape: Rectangle(5, 8),
        // {{{
        // shape: Point,
        // }}}
    };

    shnep.describe();
}

// vim: set foldmethod=marker:
