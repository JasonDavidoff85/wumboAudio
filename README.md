# wumboAudio
real time synth written in rust

# Docs

## Terminology
sound_source - `Generator` trait object aka anything that implements `Generator`  

## [Generator](src/generator.rs)
`Trait` - Outputs sample (f32) signal

**Implemented**
* `Item::Generator()`
* `f64::Generator()`

**Planned**


## [Renderer](src/mixer.rs)
`Struct` - Mixer that takes in sound sources (ie. bus, vco) held in `vec`

**Implemented**
* `Renderer::out()`
    *  Adds up vcos and outputs at volume  
* `Renderer::add_source()`
    * Adds sound source to mixer

**Planned**

## [Bus](src/bus.rs)
`Struct` - Holds list of signals to be processed sequentially ie. vco -> filter -> reverb

**Implemented**
* `Bus::push()`
    *  adds signal to chain

**Planned**
* `Bus::insert(index: usize)`
    * insert signal in chain
* Check that everything pushed can take an input
    * ie. what happens if a vco is pushed after another vco?

## [VCO](src/vco.rs)
`Struct` - sound_source

**Implemented**
* Takes in freq, volume, and wave shape
* Parameterized freq (can accept Item which is something that implements Generator)
    * allows for **FM synthesis**
* Parameterized volume

**Planned**
* ~~Parameterize volume~~
* Parameterize shape

## [LFO](src/lfo.rs)
`Struct` - sound_source. Although meant to be used as control signal level.

**Implemented**
* Takes in freq, and width
* Working generator

**Planned**
* Parameterize freq
* Parameterize width

## [Filter](src/filter.rs) - *In progress*
`Struct` - Audio filter

**Implemented**
* peaking and low pass eq functions
* Works?

**Planned**
* Parameterize filter type
* Implement into audio chain

# TODO
* Implement buses
    * Mixer takes in list of busses
    * Bus has effects at end of chain
* mixer -> processing/effects -> outd
* Create better audio chain
    * Think about filters, effects like reverb and delay

* Reverb

* CLI

* Add volume control to mixer

* Reciever trait for inputs?

# Bugs

* lfo on freq will continue to increase in volume

* an lfo on one audio source volume will also modulate another sound source's volume

