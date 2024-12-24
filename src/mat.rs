#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mat<T> {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<T>,
}

impl<T> Mat<T>
where
    T: Clone,
{
    pub fn filled_with(value: T, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            vec: vec![value; height * width],
        }
    }

    pub fn get(&self, index: (usize, usize)) -> &T {
        &self.vec[self.map_index(index)]
    }

    pub fn set(&mut self, index: (usize, usize), v: T) {
        let index = self.map_index(index);
        self.vec[index] = v;
    }

    fn map_index(&self, index: (usize, usize)) -> usize {
        index.0 + index.1 * self.width
    }
}
