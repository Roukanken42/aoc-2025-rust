pub struct DynamicZip<I>
where
    I: Iterator,
{
    iterators: Vec<I>,
}

impl<I, T> Iterator for DynamicZip<I>
where
    I: Iterator<Item = T>,
    T: std::fmt::Debug,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators.iter_mut().map(|i| i.next()).collect()
    }
}

pub trait DynamicZipable<I, T>
where
    I: Iterator<Item = T>,
{
    fn dynamic_zip(self) -> DynamicZip<I>;
}

impl<Outer, Inner, T> DynamicZipable<Inner, T> for Outer
where
    Inner: Iterator<Item = T>,
    Outer: Iterator<Item = Inner>,
{
    fn dynamic_zip(self) -> DynamicZip<Inner> {
        DynamicZip {
            iterators: self.collect::<Vec<_>>(),
        }
    }
}
