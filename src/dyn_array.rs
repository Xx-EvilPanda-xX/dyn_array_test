use std::ops::{Index, IndexMut};
pub struct DynArray<T, const D: usize> {
    dims: [usize; D],
    data: Vec<T>,
}

impl<T: Clone, const D: usize> DynArray<T, D> {
    pub fn new(dims: [usize; D], x: T) -> Self {
        let mut vec_len = 1;

        for dim in dims {
            vec_len *= dim;
        }

        Self {
            dims,
            data: std::vec::from_elem(x, vec_len),
        }
    }

    pub fn dims(&self) -> &[usize] {
        &self.dims
    }
}

impl<T, const D: usize> Index<[usize; D]> for DynArray<T, D> {
    type Output = T;
    fn index(&self, index: [usize; D]) -> &Self::Output {
        &self.data[get_index(&self.dims, &index)]
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for DynArray<T, D> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output {
        &mut self.data[get_index(&self.dims, &index)]
    }
}

fn get_index(dims: &[usize], index: &[usize]) -> usize {
    for (i, dim) in index.iter().enumerate() {
        if dim >= &dims[i] {
            panic!("index out of bounds")
        }
    }

    let mut idx = 0;

    for (i, dim) in index.iter().enumerate() {
        let mul: usize = dims[..i].iter().product();
        idx += dim * mul;
    }

    idx
}