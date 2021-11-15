use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Ord + Add<Output = T> + PartialEq + Eq + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if a + b > c && a + c > b && b + c > a {
            let triangle = Triangle { a, b, c };
            Some(triangle)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.a != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b && self.b != self.c)
            || (self.a != self.b && self.b == self.c)
            || (self.a == self.c && self.b != self.c)
    }
}
