use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Clone, Default)]
pub struct InnerState<T> {
    value: Rc<RefCell<T>>,
    ctx: StateContext,
    subscribers: Rc<RefCell<HashSet<usize>>>,
}

#[derive(Clone)]
pub struct GetState<T> {
    inner: InnerState<T>,
}

impl<T: Default> Default for GetState<T> {
    fn default() -> Self {
        GetState { inner: Default::default() }
    }
}

impl<T: Clone> GetState<T> {
    pub fn get(&self) -> T {
        if let Some(id) = *self.inner.ctx.current_effect.borrow() {
            self.inner.subscribers.borrow_mut().insert(id);
        }

        self.inner.value.borrow().clone()
    }
}

#[derive(Clone)]
pub struct SetState<T> {
    inner: InnerState<T>,
}

impl<T: Default> Default for SetState<T> {
    fn default() -> Self {
        SetState { inner: Default::default() }
    }
}

impl<T> SetState<T> {
    pub fn set(&self, new_value: T) {
        *self.inner.value.borrow_mut() = new_value;

        let subscribers: Vec<_> = self.inner.subscribers.borrow().iter().cloned().collect();

        for id in subscribers {
            if let Some(effect) = self.inner.ctx.effects.borrow().get(&id) {
                effect();
            }
        }
    }

    pub fn update<F>(&self, mut update_fn: F)
    where
        F: FnMut(&T) -> T,
    {
        let current = self.inner.value.borrow();
        let new_value = update_fn(&current);
        drop(current); // important !!!! release immutable borrow
        *self.inner.value.borrow_mut() = new_value;

        let subscribers: Vec<_> = self.inner.subscribers.borrow().iter().cloned().collect();

        for id in subscribers {
            if let Some(effect) = self.inner.ctx.effects.borrow().get(&id) {
                effect();
            }
        }
    }
}

pub fn use_state<T: Clone>(ctx: StateContext, initial: T) -> (GetState<T>, SetState<T>) {
    let inner = InnerState {
        value: Rc::new(RefCell::new(initial)),
        ctx: ctx.clone(),
        subscribers: Rc::new(RefCell::new(HashSet::new())),
    };

    (GetState { inner: inner.clone() }, SetState { inner: inner.clone() })
}

pub fn use_effect<F: Fn() + 'static>(ctx: StateContext, f: F) {
    let id = ctx.next_effect_id.get();

    ctx.next_effect_id.set(id + 1);

    let f = Rc::new(f);
    ctx.effects.borrow_mut().insert(id, f.clone());

    let wrapped = {
        let ctx = ctx.clone();

        move || {
            *ctx.current_effect.borrow_mut() = Some(id);
            f();
            *ctx.current_effect.borrow_mut() = None;
        }
    };

    wrapped(); // Run immediately
}

#[derive(Clone, Default)]
pub struct StateContext {
    current_effect: Rc<RefCell<Option<usize>>>,
    effects: Rc<RefCell<HashMap<usize, Rc<dyn Fn()>>>>,
    next_effect_id: Rc<Cell<usize>>,
}

impl StateContext {
    pub fn new() -> Self {
        StateContext {
            current_effect: Rc::new(RefCell::new(None)),
            effects: Rc::new(RefCell::new(HashMap::new())),
            next_effect_id: Rc::new(Cell::new(0)),
        }
    }
}
