use crate::{vco::VCO, generator::Generator};

/// This is a linked list that contains an audio chain
/// every item is a generator and each item must have an input
/// It is processed in sequential order and there is no adding
/// A bus represents a single audio source/voice
pub struct Bus {
    time: f64,
    pub signals: Vec<Box<dyn Generator<Item = f32>>>
}

// TODO implement panning
impl Bus {
    pub fn new() -> Self {
        return Bus { time: 0., signals: Vec::new() }
    }

    /// adds a signal to the chain 
    /// pushes to back of chain
    pub fn push<T: Generator<Item=f32> + 'static>(&mut self, signal: T) -> usize {
        self.signals.push( Box::new(signal) );
        self.signals.len() + 1
    }
}

/// Generates a signle sample run though the entire chain
impl Generator for Bus {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1. / 44_100.;
        let mut output = 0f32;
        for signal in self.signals.iter_mut() {
            output = signal.next().unwrap();
        }
        Some(output)
    }
}