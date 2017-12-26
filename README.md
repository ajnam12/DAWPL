# DAWPL
DAWPL stands for "digital audio workstation programming language". It provides 
abstractions and constructs that facilitate algorithmic composition. DAWPL's
music theory abstractions allow composers to interact directly with a familiar
representation that has well-established rules and conventions for writing 
music that "sounds good" in the traditional sense. The DAW-like structural
organization (explained later) forces the user to organize their code in a
particular way -- making it more readable and modular.

# The 5-Layer DAWPL Abstraction Hierarchy
**Arrangement:** a combination of tracks, the "big picture" of the composition

**Track:** a time sequenced series of clips

**Clip:** a combination of signals

**Signal:** a description of a sound e.g. a waveform or mp3 file

**Audio:** the physical production of sound

# Music Theory Abstractions
**Chord:** A combination of notes, often defined relative to a particular scale

**Scale:** A sequence of notes given by an interval-sequence formula

**Note:** A tone of a certain frequency

# SuperCollider
DAWPL translates to SuperCollider, a popular and powerful DSP-oriented language
for algorithmic music composition. DAWPL is intended as a bridge between the
audio/DSP power of SuperCollider and the syntactic power of Rust, its macro
system, and DAWPL's music theory and organizational abstractions.

# Possible Use Cases
DAWPL can be used to let artists "prototype" tunes and productions. Artists can
also use it to build tools that faciliate compositions (e.g. an auto-improviser
that comes up with new melodies according to a rule conveyed in code). Through
interacting with the rich and complex structure of music in a programmatic
fashion, artists have the potential to rapidly develop and realize their ideas.

# This is still a work in progress
I plan to add audio effects, support for reading audio file formats, and
improved documentation. Please let me know if you're interested in helping out
with this project and if you have any advice for what could be added to it! If
you're a musician/producer, please let me know what other kinds of patterns and
constructs you would like support for!


