/* Make it work*/
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NOTE: the use of the doubly curlies here (`{{}}`),
        // this is used to escape the curlies and
        // display them instead of reading their values
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn main() {
    let point: Point2D = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(
        format!("{:?}", point),
        "Debug: Complex { real: 3.3, imag: 7.2 }"
    );

    println!("{}", point);
    println!("{:?}", point);

    println!("Success!");
}
