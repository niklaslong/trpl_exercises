#[derive(Debug)]
pub struct Rectangle<T, U> {
    pub width: T,
    pub length: U,
}

impl<T, U> Rectangle<T, U> {
    pub fn new(width: T, length: U) -> Rectangle<T, U> {
        Rectangle {
            width: width,
            length: length,
        }
    }
}

use std::ops::Add;

impl<T: Add<Output = T>, U: Add<Output = U>> Add for Rectangle<T, U> {
    type Output = Rectangle<T, U>;

    fn add(self, other: Rectangle<T, U>) -> Rectangle<T, U> {
        Rectangle {
            width: self.width + other.width,
            length: self.length + other.length,
        }
    }
}

impl<T, U> Rectangle<T, U>
where
    T: PartialOrd,
    U: PartialOrd,
{
    pub fn can_hold(&self, other: &Rectangle<T, U>) -> bool {
        // comparison operators require the two sides to be of same type
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(3, 4);
        let smaller = Rectangle::new(2, 1);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle::new(3, 4.1);
        let smaller = Rectangle::new(1, 2.1);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn adds_two_rectangles() {
        let rect1 = Rectangle::new(3, 4);
        let rect2 = Rectangle::new(1, 2);

        let result = rect1.add(rect2);

        assert_eq!(result.width, 4);
        assert_eq!(result.length, 6);
    }
}
