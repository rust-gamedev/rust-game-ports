use std::{cell::RefCell, rc::Rc};

pub type RCC<T> = Rc<RefCell<T>>;

pub fn new_rcc<T>(t: T) -> RCC<T> {
    Rc::new(RefCell::new(t))
}
