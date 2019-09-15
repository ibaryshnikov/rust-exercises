#![feature(fn_traits, unboxed_closures)]

struct Data {
    value: i32,
}

impl FnOnce<()> for Data {
    type Output = i32;
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.value
    }
}

impl FnMut<()> for Data {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.value += 1;
        self.value
    }
}

fn main() {
    let mut closure = Data { value: 0 };
    println!("{}", closure());
    println!("{}", closure());
}
