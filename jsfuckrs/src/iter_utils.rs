pub trait IteratorUtils: Iterator {
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }
}

impl<T: ?Sized> IteratorUtils for T where T: Iterator {}
