use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Container {
    value: Point,
}

impl Deref for Container {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for Container {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut container = Container { value: Point { x: 1, y: 2 } };
    println!("{:?}", container);
    println!("{:?}", *container);
    container.x += 2;
    println!("{} {}", container.x, container.y);
}
