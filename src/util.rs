use std::{
    iter::Enumerate,
    slice::{
        Iter,
        IterMut,
    },
};

pub fn enumiter<T>(obj: &Vec<T>) -> Enumerate<Iter<'_, T>> {
    obj.iter().enumerate()
}

pub fn enumiter_mut<T>(obj: &mut Vec<T>) -> Enumerate<IterMut<'_, T>> {
    obj.iter_mut().enumerate()
}
