use std::ops::Range;

fn clamp_usize_range(range: Range<usize>, clamp: usize) -> Range<usize> {
    match range.end > clamp {
        true => range.start..clamp,
        false => range,
    }
}

pub struct Array2D<T> {
    width: usize,
    height: usize,
    data: Box<[T]>,
}

impl<T> Array2D<T> {
    pub fn from_closure(width: usize, height: usize, ctor: impl Fn(usize, usize) -> T) -> Self {
        let mut arr = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                arr.push(ctor(i, j));
            }
        }

        Self {
            width,
            height,
            data: arr.into_boxed_slice(),
        }
    }

    pub fn from_box(width: usize, height: usize, data: Box<[T]>) -> Self {
        assert!(width * height == data.len());

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn index_get(&self, index: usize) -> Option<&T> {
        match index < self.width * self.height {
            false => None,
            true => Some(&self.data[index]),
        }
    }

    pub fn index_get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.width * self.height {
            false => None,
            true => Some(&mut self.data[index]),
        }
    }

    pub fn coord_get(&self, i: usize, j: usize) -> Option<&T> {
        match i * self.width * j < self.width * self.height {
            false => None,
            true => Some(&self.data[i * self.width * j]),
        }
    }

    pub fn coord_get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        match i * self.width * j < self.width * self.height {
            false => None,
            true => Some(&mut self.data[i * self.width * j]),
        }
    }

    pub fn for_each(&self, mut func: impl FnMut(usize, usize, &T)) {
        for j in 0..self.height {
            let j_offset = j * self.width;
            for i in 0..self.width {
                func(i, j, &self[i + j_offset]);
            }
        }
    }

    pub fn for_each_mut(&mut self, mut func: impl FnMut(usize, usize, &mut T)) {
        for j in 0..self.height {
            let j_offset = j * self.width;
            for i in 0..self.width {
                func(i, j, &mut self[i + j_offset]);
            }
        }
    }

    pub fn for_each_sub(
        &self,
        i_range: Range<usize>,
        j_range: Range<usize>,
        mut func: impl FnMut(usize, usize, &T),
    ) {
        // clamp ranges
        let i_range = clamp_usize_range(i_range, self.width);
        let j_range = clamp_usize_range(j_range, self.height);

        for j in j_range {
            let j_offset = j * self.width;
            for i in i_range.clone() {
                func(i, j, &self[i + j_offset]);
            }
        }
    }

    pub fn for_each_sub_mut(
        &mut self,
        i_range: Range<usize>,
        j_range: Range<usize>,
        mut func: impl FnMut(usize, usize, &mut T),
    ) {
        // clamp ranges
        let i_range = clamp_usize_range(i_range, self.width);
        let j_range = clamp_usize_range(j_range, self.height);

        for j in j_range {
            let j_offset = j * self.width;
            for i in i_range.clone() {
                func(i, j, &mut self[i + j_offset]);
            }
        }
    }
}

impl<T: Clone> Array2D<T> {
    pub fn clone(&self) -> Array2D<T> {
        Array2D::from_closure(self.width, self.height, |i, j| self[(i, j)].clone())
    }

    pub fn clone_sub(&self, i_range: Range<usize>, j_range: Range<usize>) -> Array2D<T> {
        // clamp ranges
        let i_range = clamp_usize_range(i_range, self.width);
        let j_range = clamp_usize_range(j_range, self.height);

        Array2D::from_closure(
            i_range.end - i_range.start,
            j_range.end - j_range.start,
            |i, j| self[(i_range.start + i, j_range.start + j)].clone(),
        )
    }
}

impl<T> std::ops::Index<usize> for Array2D<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.width]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * self.width]
    }
}
