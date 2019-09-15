use std::rc::Rc;
use std::cell::RefCell;

struct Data {
    value: i32,
}

fn get_closure(data: Rc<RefCell<Data>>) -> impl FnMut() -> i32 {
    move || {
        data.borrow_mut().value += 1;
        data.borrow().value
    }
}

fn main() {
    let data = Rc::new(RefCell::new(Data { value: 0 }));
    let mut closure_1 = get_closure(data.clone());
    let mut closure_2 = get_closure(data.clone());
    
    println!("{}", closure_1());
    println!("{}", closure_1());
    println!("{}", closure_2());
    println!("{}", closure_1());
    println!("{}", closure_2());
}
