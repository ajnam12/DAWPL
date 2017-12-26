/// This file defines a set of macros to provide easy and convenient access
/// to DAWPL's abstractions

use arrangement::*;
use chord::*;
use clip::*;
use note::*;
use scale::*;
use track::*;

/// This macro is used to conveniently define notes e.g. n!(C4), n!(Ds4)
macro_rules! n {
    ($note:ident) => {{
        let name_octave: String = String::from(stringify!($note));
        let result_note: Note = name_octave.parse().unwrap();
        result_note
    }}
}

/// The chord macro can be used to define chords like so:
/// chord!(C4, Maj7) defines a Cmaj7 chord with C4 as the root
macro_rules! chord {
    ($note:ident,$chord_type:ident) => {{
        let root = n!($note);
        let chord_type: ChordType = String::from(stringify!($chord_type))
        .parse().unwrap();
        Chord::new(root, chord_type)
    }}
}

/// Shorthand to express an instrument clip by denoting a name for the clip,
/// the name of the desired instrument, a list of notes (MIDI), and their
/// respective durations.
macro_rules! instr_clip {
    ($clip_name:ident, $instr_name:ident, $notes:expr, $durations:expr) => {{
        Clip::Instrument(String::from(stringify!($clip_name)),
            String::from(stringify!($instr_name)),
            $notes, $durations)
    }}
}

/// Allows user to specify a track by giving it a name and enumerating its
/// clip names e.g. track!(<track-name>, <clip_0>, ..., <clip_n>)
macro_rules! track {
    ($name:ident, $( $clip_name:ident),*) => {{
        let mut clip_names: Vec<String> = Vec::new();
        $(
            clip_names.push(String::from(stringify!($clip_name)));
        )*
        Track::new(String::from(stringify!($name)), clip_names)
    }}
}
/// A macro that maps letters to durations, which can be element-wise multiplied
/// to align with different tempos (might later work tempo into the macro)
macro_rules! rhythm {
    ($( $beat:ident),*) => {{
        let mut durations: Vec<f64> = Vec::new();
        $( 
            let next_dur: f64 = match stringify!($beat) {
                "W" => 1.0,
                "H" => 0.5,
                "Q" => 0.25,
                "E" => 0.125,
                "S" => 0.0625,
                _ => 0.0,
            };
            durations.push(next_dur);
        )*
        durations
    }}
}
// TODO: macros for defining synths => layer other aspects on top of SuperCollider
// TODO: add audio effects.

// TODO: arrangement macro

impl Playable for () { // to quiet compiler complaints
    fn play(&self) -> Vec<i8> {
        Vec::new()
    } 
}

/// The play macro can be used to conveniently define instrumental clips.
/// An empty pair of parens "()" can be used to denote rests.
macro_rules! play {
    ($( $notes:expr),*) => {{
        let mut midi_values: Vec<Option<Vec<i8>>> = Vec::new();
        $( 
            //println!("Notes stringified: {}", stringify!($notes));
            let elem: Option<Vec<i8>> = match stringify!($notes) {
                "()" => None,
                _ => Some($notes.play()),
            };
            midi_values.push(elem);
        )*
        midi_values
    }}

}


// TODO: play! macro: play![a, b, c] => vec![a.play(), b.play(), c.play()]