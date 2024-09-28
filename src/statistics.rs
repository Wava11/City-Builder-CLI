pub trait Sample<T> {
    fn sample(&mut self, amount: u64) -> Vec<T>;
}
pub trait SampleBasedOn<T> {
    fn sample_based_on(&mut self, t: &T) -> T;
}
