use std::ops::{Index, IndexMut};

use super::{Vector, VectorElement};

macro_rules! vector_index_reload_impl {
    ($vec_name:ident) => {
        impl<T: VectorElement> Index<usize> for $vec_name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.value[index]
            }
        }

        impl<T: VectorElement> IndexMut<usize> for $vec_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.value[index]
            }
        }
    };
}

vector_index_reload_impl!(Vector);
