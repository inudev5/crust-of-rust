use std::cell::{UnsafeCell};
use std::ops::{Deref, DerefMut};
use crate::cell::Cell;
use crate::RefCell::RefState::Unshared;

#[derive(Copy, Clone)] //Cell은 get()시에 참조가 아닌 Copy를 제공
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        use RefState::*;
        match self.state.get() {
            Unshared => {
                self.state.set(Shared(1));
                Some(Ref{refcell:self})
            }
            Shared(n) => {
                self.state.set(Shared(n + 1)); //thread-unsafe!!
                Some(Ref{refcell:self})
            }
            Exclusive => None
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut{refcell:self})
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        use RefState::*;
        match self.refcell.state.get() {
            Exclusive | Unshared => unreachable!(), //공유참조일 때 exclusive 일 수 없음
            Shared(1) => self.refcell.state.set(Unshared),
            Shared(n) => self.refcell.state.set(Shared(n - 1)),
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_,T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.refcell.value.get()}
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {&*self.refcell.value.get()}
    }
}
impl<T> DerefMut for RefMut<'_, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.refcell.value.get()}
    }
}


impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        use RefState::*;
        match self.refcell.state.get() {
            Shared(_) | Unshared => unreachable!(), //공유참조일 때 exclusive 일 수 없음
            Exclusive => self.refcell.state.set(Unshared),
        }
    }
}
