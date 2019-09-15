fn get_closure() -> impl FnMut() -> i32 {
    let mut total = 0;
    move || {
        total += 1;
        total
    }
}

fn main() {
    let mut closure_1 = get_closure();
    println!("{}", closure_1());
    println!("{}", closure_1());
    println!("{}", closure_1());
    let mut closure_2 = get_closure();
    println!("{}", closure_2());
}
