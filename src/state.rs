use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct GetState<T> {
    inner: Rc<RefCell<T>>,
}

impl<T: Default> Default for GetState<T> {
    fn default() -> Self {
        GetState { inner: Default::default() }
    }
}

impl<T: Clone> GetState<T> {
    pub fn get(&self) -> T {
        self.inner.borrow().clone()
    }
}

#[derive(Clone)]
pub struct SetState<T> {
    inner: Rc<RefCell<T>>,
}

impl<T: Default> Default for SetState<T> {
    fn default() -> Self {
        SetState { inner: Default::default() }
    }
}

impl<T> SetState<T> {
    pub fn set(&self, new_value: T) {
        *self.inner.borrow_mut() = new_value;
    }

    pub fn update<F>(&self, mut update_fn: F)
    where
        F: FnMut(&T) -> T,
    {
        let current = self.inner.borrow();
        let new_value = update_fn(&current);
        drop(current); // important !!!! release immutable borrow
        *self.inner.borrow_mut() = new_value;
    }
}

pub fn use_state<T>(initial: T) -> (GetState<T>, SetState<T>) {
    let rc = Rc::new(RefCell::new(initial));
    (GetState { inner: rc.clone() }, SetState { inner: rc })
}




pub struct StateContext {
}

impl StateContext {
    pub fn new() -> Self {
        StateContext {  }
    }
}
