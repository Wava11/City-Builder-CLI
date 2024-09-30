pub trait Sample<T> {
    fn sample(&mut self, amount: u64) -> Vec<T>;
}
pub trait SampleBasedOn<Base, Out> {
    fn sample_based_on(&mut self, t: &Base, amount: u64) -> Vec<Out>;
}
