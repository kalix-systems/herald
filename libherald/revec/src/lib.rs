use im::vector::Vector;

#[derive(Clone, Debug)]
pub struct Revec<T: Clone>(Vector<T>);

impl<T: Clone> Default for Revec<T> {
    fn default() -> Self {
        Self(Vector::new())
    }
}

impl<T: Clone> From<Vec<T>> for Revec<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().collect())
    }
}

impl<T: Clone> Revec<T> {
    fn t(
        &self,
        ix: usize,
    ) -> usize {
        self.0.len().saturating_sub(1 + ix)
    }

    pub fn get(
        &self,
        ix: usize,
    ) -> Option<&T> {
        self.0.get(self.t(ix))
    }

    pub fn get_mut(
        &mut self,
        ix: usize,
    ) -> Option<&mut T> {
        self.0.get_mut(self.t(ix))
    }

    pub fn insert_ord(
        &mut self,
        v: T,
    ) -> usize
    where
        T: Ord,
    {
        match self.0.binary_search(&v) {
            Ok(index) => {
                self.0.insert(index, v);
                self.t(index)
            }
            Err(index) => {
                self.0.insert(index, v);
                self.t(index)
            }
        }
    }

    pub fn last(&self) -> Option<&T> {
        self.0.front()
    }

    pub fn front(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn remove(
        &mut self,
        ix: usize,
    ) -> Option<T> {
        let ix = self.t(ix);
        if ix >= self.0.len() {
            return None;
        }

        Some(self.0.remove(ix))
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &T> + DoubleEndedIterator<Item = &T> {
        self.0.iter().rev()
    }

    pub fn iter_mut(
        &mut self
    ) -> impl ExactSizeIterator<Item = &mut T> + DoubleEndedIterator<Item = &mut T> {
        self.0.iter_mut().rev()
    }

    pub fn binary_search(
        &self,
        val: &T,
    ) -> Result<usize, usize>
    where
        T: Ord,
    {
        match self.0.binary_search(val) {
            Ok(ix) => Ok(self.t(ix)),
            Err(ix) => Err(self.t(ix)),
        }
    }
}
