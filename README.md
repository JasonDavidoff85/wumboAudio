# wumboAudio
real time synth written in rust

# Docs

## [Renderer](src/mixer.rs)
`Struct` - Holds list of VCOs and outputs sum

**Implemented**
* `Renderer::out()`
    *  Adds up vcos and outputs at volume  

**Planned**


## [Generator](src/generator.rs)
`Trait` - Outputs samples to be processed by Renderer

**Implemented**
* `Item::Generator()`
* `f64::Generator()`

**Planned**

## [VCO](src/vco.rs)
`Struct` - Audio source

**Implemented**
* Takes in freq, volume, and wave shape
* Parameterized freq (can accept Item which is something that implements Generator)
    * allows for **FM synthesis**

**Planned**
* Parameterize volume
* Parameterize shape

## [LFO](src/lfo.rs)
`Struct` - Generator implementor. Generate low freq signals for parameter modulation

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
* mixer -> processing/effects -> outd
* Create better audio chain
    * Think about filters, effects like reverb and delay

* Reverb

* CLI



