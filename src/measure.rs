pub trait Measure : Ord {
    fn zero() -> Self;
    fn infinity() -> Self;
}