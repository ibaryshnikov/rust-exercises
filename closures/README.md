## Closures

There are three exercises here.

- Implement `FnOnce`, `FnMut` and `Fn` traits for a struct and invoke it as a regular function.
  Example of struct is
  ```rust
  struct Data {
      value: i32,
  }
  ```
  Usage:
  ```rust
  let mut closure = Data { value: 0 };
  closure();
  closure();
  ```
  Currently requires nightly. The answer is in [fn-trait.rs](./fn-trait.rs). Use `rustc +nightly fn-trait.rs` to build

- Return a closure from a function and count a number of invocations
  ```rust
  let mut closure_1 = get_closure(); // create a closure
  println!("{}", closure_1()); // 1
  println!("{}", closure_1()); // 2
  println!("{}", closure_1()); // 3
  let mut closure_2 = get_closure(); // create another closure
  println!("{}", closure_2()); // 1
  ```
  The answer is in [simple-capture.rs](simple-capture.rs)

- Return a closure which will hold a shared resource
  ```rust
  let data = Rc::new(RefCell::new(Data { value: 0 }));
  let mut closure_1 = get_closure(data.clone()); // create a closure
  let mut closure_2 = get_closure(data.clone()); // create another closure
  println!("{}", closure_1()); // 1
  println!("{}", closure_1()); // 2
  println!("{}", closure_2()); // 3, notice that we call the second closure here
  println!("{}", closure_1()); // 4, calling the first closure again
  println!("{}", closure_2()); // 5
  ```
  The answer is in [rc-refcell-closure.rs](./rc-refcell-closure.rs)
