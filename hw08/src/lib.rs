use std::f64::consts::PI;

pub struct Triangle {
    pub sides_lens: [f64; 3],
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub trait Shape {
    fn get_area(&self) -> f64;
    fn get_perimeter(&self) -> f64;
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        let semiperimeter = self.get_perimeter() / 2.0;
        let area_squared = semiperimeter
            * self
                .sides_lens
                .iter()
                .map(|&side| semiperimeter - side)
                .product::<f64>();
        area_squared.sqrt()
    }

    fn get_perimeter(&self) -> f64 {
        self.sides_lens.iter().sum()
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

pub fn perimeter_by_area(shape: impl Shape) -> f64 {
    shape.get_perimeter() / shape.get_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, relative_eq};

    #[test]
    fn test_triangle_perimeter() {
        let t = Triangle {
            sides_lens: [2.0, 3.0, 4.0],
        };
        let perimeter = t.get_perimeter();
        assert_eq!(perimeter, 9.0);
    }

    #[test]
    fn test_triangle_area() {
        let t = Triangle {
            sides_lens: [2.0, 3.0, 4.0],
        };
        let area = t.get_area();

        assert_abs_diff_eq!(area, 2.90473750, epsilon = 1e-8)
    }

    #[test]
    fn test_rectangle_perimeter() {
        let r = Rectangle {
            width: 2.0,
            height: 4.0,
        };
        let perimeter = r.get_perimeter();
        assert_eq!(perimeter, 12.0);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle {
            width: 2.0,
            height: 4.0,
        };
        let area = r.get_area();

        assert_eq!(area, 8.0)
    }

    #[test]
    fn test_circle_perimeter() {
        let c = Circle { radius: 2.0 };
        let perimeter = c.get_perimeter();

        assert_abs_diff_eq!(perimeter, 12.56637061, epsilon = 1e-8)
    }

    #[test]
    fn test_circle_area() {
        let c = Circle { radius: 2.0 };
        let area = c.get_area();

        assert_abs_diff_eq!(area, 12.56637061, epsilon = 1e-8)
    }

    #[test]
    fn test() {
        relative_eq!(
            perimeter_by_area(Triangle {
                sides_lens: [3.0, 4.0, 5.0]
            }),
            2.0
        );
        relative_eq!(perimeter_by_area(Circle { radius: 2.0 }), 1.0);
        relative_eq!(
            perimeter_by_area(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
            1.6666
        );
    }
}
