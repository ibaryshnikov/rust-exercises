## Futures

There are two exercises here.

- Implement `std::future::Future` trait for a struct and use `.await` on it.
  Example of struct is
  ```rust
  struct Data {
      value: i32,
  }
  ```
  Usage:
  ```rust
  let data = Data { value: 42 };
  let answer = data.await;
  println!("The answer is {}", answer);
  ```
  The answer is in [main.rs](./impl-future-for-a-struct/src/main.rs).

- Use `futures::task::AtomicWaker` to wake a future from another thread after
  a short delay. Currently in progress, will be posted later.
