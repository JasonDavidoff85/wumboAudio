pub trait Generator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Generator for f32 {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}
