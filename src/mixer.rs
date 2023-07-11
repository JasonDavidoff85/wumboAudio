use crate::{vco::VCO, generator::Generator};

/// This is an object that spits out a left and right channel
/// it takes in every kind of audio producer and mixes to desired output
/// will eventually applys filters and stuff appropiately
pub struct Renderer {
    volume: f32,
    pub audio_sources: Vec<Box<dyn Generator<Item = f32>>>
}

// TODO implement panning
impl Renderer {
    pub fn new(volume: f32) -> Self {
        return Renderer { volume, audio_sources: Vec::new() }
    }

    /// adds a sound source to the mixer
    /// order does not matter, all sources are mixed at level
    pub fn add_source<T: Generator<Item=f32> + 'static>(&mut self, signal: T) -> usize {
        self.audio_sources.push( Box::new(signal) );
        self.audio_sources.len() + 1
    }

    pub fn out(&mut self) -> f32 {
        let mut sample = 0.;
        for source in self.audio_sources.iter_mut() {
            sample += source.next().unwrap();
        }
        return sample as f32 * self.volume;
    }
}