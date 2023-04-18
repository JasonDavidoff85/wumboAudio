pub trait Generator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Generator for f64 {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}
