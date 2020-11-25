#[derive(Clone)]
pub struct Matrix<I> where I: Iterator {
    pub iterators: Vec<I>
}

impl<I, T> Iterator for Matrix<I>
    where I: Iterator<Item = T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Vec<T>> = self.iterators.iter_mut().map(|iter| iter.next()).collect();
        output
    }
}
