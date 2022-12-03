pub trait CollectInto<T, E: Extend<T>> {
    fn collect_into(self, collector: &mut E) -> &mut E;
}

impl<I, T, E> CollectInto<T, E> for I 
where
    I: Iterator<Item = T>,
    E: Extend<T>,
{
    fn collect_into(self, collector: &mut E) -> &mut E {
        collector.extend(self);
        collector
    }
}