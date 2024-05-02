// {{{
#![allow(dead_code, clippy::upper_case_acronyms)]
// }}}

// NOTE: Simple "C-like" enum, variants have no attached data
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// NOTE: Simple "C-like" struct
struct Person {
    name: String,
    age: u16,
}

// NOTE: Anonymous struct, fields get automatic names `0`, `1` and so on
//       Same as a regular struct otherwise
struct Anonymous(u8, String, DayOfWeek);

// {{{

// NOTE: struct-like and tuple-like variants
#[derive(Debug)]
enum Colour {
    RGB { r: u8, g: u8, b: u8 },
    Grayscale(u8),
}

enum Shape {
    // Point,
    Square { side: u8 },
    Cirlce { radius: f32 },
    Rectangle(u8, u8),
}

// {{{
impl Shape {
    // NOTE: `self` vs `&self` vs `&mut self`
    fn name(&self) -> &str {
        match self {
            // Shape::Point => "point",
            Shape::Square { .. } => "square",
            Shape::Cirlce { .. } => "circle",
            Shape::Rectangle(..) => "rectangle",
        }
    }

    fn area(&self) -> f32 {
        match self {
            // Shape::Point => 0_f32,
            Shape::Square { side } => (side * side) as f32,
            Shape::Cirlce { radius } => std::f32::consts::PI * radius * radius,
            Shape::Rectangle(a, b) => *a as f32 * *b as f32,
        }
    }
}
// }}}

struct ColouredShape {
    colour: Colour,
    shape: Shape,
}

// {{{
impl ColouredShape {
    fn describe(&self) {
        println!(
            "This shape is a {:?} {:?} and has an area of {:?}",
            self.colour,
            self.shape.name(),
            self.shape.area(),
        );
    }
}
// }}}

pub fn main() {
    let shape = ColouredShape {
        colour: Colour::Grayscale(123),
        shape: Shape::Rectangle(5, 8),
    };

    shape.describe();
}

// }}}

// vim: set foldmethod=marker:
