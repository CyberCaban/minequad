use std::ops::{Deref, DerefMut, Index};

struct BigVec<T>(Vec<T>);

impl<T> Deref for BigVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for BigVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> Index<i32> for BigVec<T> {
    type Output = T;

    fn index(&self, index: i32) -> &T {
        if index < 0 || index >= self.0.len() as i32 {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.0.len(),
                index
            );
        }

        &self.0[index as usize]
    }
}

