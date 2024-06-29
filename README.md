# wumboAudio
real time synth written in rust

# Docs

## CLI reference

At start up, a mixer is automatically created and can be referred to with `mixer`

`render` will call the render function of the mixer and play audio

### **quick start:**

<br>

*hello world*
```
create vco --name helloWorld 440 0.8 sine
add helloWorld to mixer 
render
```

<hr>

*using a bus*
```
create bus --name myBus
add bus to mixer
create vco 440 0.9 square | myBus
create filter 280 0.2 | myBus
render
show myBus
myBus.chain[0].freq = 340
myBus.vol = 0.5
render
```

`show myBus` output:
```
object: myBus
type: Bus
attributes: {
    vol: 0.8
    chain: [vco(id:2135)]
    processing: [filter(id:3245)]
}
attached to: mixer
```

<hr>

*FM synthesis*
```
create bus 0.4 | mixer
create vco 200 0.2 square --name drone | mixer
show mixer
drone.vol = 0.1
create vco | mixer[0] // add vco to bus
create lfo 5 0.3 sine | mixer[0].chain[0].freq \\ add lfo to modulate vco
show mixer[0].chain[0]
render
```

`show mixer` output
```
object: id:1
type: Mixer
attributes: {
    self: [bus(id:8732), vco(drone)]
}
attached to: [audio out device]
```

`show mixer[0].chain[0]` output
```
object: id:7485
type: VCO
attributes: {
    vol: 0.8
    freq: lfo(id:4357)
    wave: sine
}
attached to: bus(id:8732)
```

<br>

`<object>` - generic object, ie. `lfo`, `vco`, `filter`. Keyword used to refer to object type

`<object-ref>` - reference to created object. can be name, or array index

`<params>` - params for an object eg. freq, volume

<hr>
<br>

| command | Descp |
| ------- | ---- |
| `list` `<object-ref>` \| objects \| `<object>` | list object info or all objects |
| `create` `<object>` [--name] `<params>` | creates object  |
| `destroy` `<object-ref>`| destory object |
| `show` `<object-ref>` | show info about object |
| `add` `<object-ref>` to `<object-ref>` | adds object ref to object |
| `render` | render all sources added to mixer |
| `\|` | short hand `add to`|

<hr>
<br>

## Code reference
<br>

### Terminology
sound_source - `Generator` trait object aka anything that implements `Generator`

<hr>
<br>

### [Generator](src/generator.rs)
`Trait` - Outputs sample (f32) signal

**Implemented**
* `Item::Generator()`
* `f64::Generator()`

**Planned**

<br>

### [Renderer](src/mixer.rs)
`Struct` - Mixer that takes in sound sources (ie. bus, vco) held in `vec`

**Implemented**
* `Renderer::out()`
    *  Adds up vcos and outputs at volume  
* `Renderer::add_source()`
    * Adds sound source to mixer

**Planned**

<br>

### [Bus](src/bus.rs)
`Struct` - Holds list of signals to be processed sequentially ie. vco -> filter -> reverb

**Implemented**
* `Bus::push()`
    *  adds signal to chain

**Planned**
* `Bus::insert(index: usize)`
    * insert signal in chain
* Check that everything pushed can take an input
    * ie. what happens if a vco is pushed after another vco?

<br>

### [VCO](src/vco.rs)
`Struct` - sound_source

**Implemented**
* Takes in freq, volume, and wave shape
* Parameterized freq (can accept Item which is something that implements Generator)
    * allows for **FM synthesis**
* Parameterized volume

**Planned**
* ~~Parameterize volume~~
* Parameterize shape

<br>

### [LFO](src/lfo.rs)
`Struct` - sound_source. Although meant to be used as control signal level.

**Implemented**
* Takes in freq, and width
* Working generator

**Planned**
* Parameterize freq
* Parameterize width

<br>

### [Filter](src/filter.rs) - *In progress*
`Struct` - Audio filter

**Implemented**
* peaking and low pass eq functions
* Works?

**Planned**
* Parameterize filter type
* Implement into audio chain

<br>

# TODO
* Implement buses
    * Mixer takes in list of busses
    * Bus has effects at end of chain
* mixer -> processing/effects -> outd
* Create better audio chain
    * Think about filters, effects like reverb and delay

* Bus error handeling

* Reverb

* CLI - use https://crates.io/crates/nom

* Add volume control to mixer

* Reciever trait for inputs?

# Bugs

* use graph: https://github.com/fluffysquirrels/rt-graph-rs/

* lfo on freq will continue to increase in volume

* an lfo on one audio source volume will also modulate another sound source's volume

