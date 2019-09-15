## Deref

- Implement `Deref` and `DerefMut` traits for a `Container` struct
  ```rust
  struct Point {
      x: i32,
      y: i32,
  }
  
  struct Container {
      value: Point,
  }
  ```
  to make the following code work
  ```rust
  let mut container = Container { value: Point { x: 1, y: 2 } };
  println!("{:?}", container); // prints Container { value: Point { x: 1, y: 2 } }
  println!("{:?}", *container); // prints Point { x: 1, y: 2 }
  container.x += 2;
  println!("{} {}", container.x, container.y); // prints 3 2
  ```
