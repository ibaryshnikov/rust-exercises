## Deref

- There's an `Option` enum in Rust which can be either `Some(value)` or `None`.
  Implement custom enum `Maybe` which will have `Just(value)` and `Nothing` variants.
  Make it possible to use `?` operator with `Maybe`, so the following code would work
  ```rust
  fn get_data(i: i32) -> Maybe<i32> { // `Maybe` is generic
      if i > 5 {
          Just(i) // top level `Just(i)` in the same way as `Some(i)`
      } else {
          Nothing // the same as `None`
      }
  }
  
  fn middle_fn(i: i32) -> Maybe<i32> {
      let inner = get_data(i)?; // notice that we can use `?` on a `Maybe` type here
      println!("{:?}", inner); // prints inner value of the `Maybe` only when it's `Just(value)`
      Just(inner)
  }
  
  fn main() {
      let data1 = middle_fn(3);
      println!("{:?}", data1); // prints Nothing
      let data2 = middle_fn(10);
      println!("{:?}", data2); // prints `Just(10)`
  }
  ```
  It requires nightly compiler, the answer is in [main.rs](./main.rs).
  Use `rustc +nightly main.rs` to build
