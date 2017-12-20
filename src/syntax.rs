// This file defines a domain specific language (DSL) for DAWPL through
// Rust's macro system
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

macro_rules! chord {
    ($note:ident,$chord_type:ident) => {{
        let root = n!($note);
        let chord_type: ChordType = String::from(stringify!($chord_type))
        .parse().unwrap();
        Chord::new(root, chord_type)
    }}
}

macro_rules! instr_clip {
    ($clip_name:ident, $instr_name:ident, $notes:expr, $durations:expr) => {{
        Clip::Instrument(String::from(stringify!($clip_name)),
            String::from(stringify!($instr_name)),
            $notes, $durations)
    }}
}

macro_rules! track {
    ($name:ident, $( $clip_name:ident),*) => {{
        let mut clip_names: Vec<String> = Vec::new();
        $(
            clip_names.push(String::from(stringify!($clip_name)));
        )*
        Track::new(String::from(stringify!($name)), clip_names)
    }}
}
// Can element-wise multiply to get different speeds
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

// based on the doc-exaple of the vec macro
// https://doc.rust-lang.org/1.7.0/book/macros.html
// macro_rules! play { // TODO: add rests, token tree type might be useful?
//     ($( $notes:tt),*) => {{
//         let mut midi_values: Vec<Option<Vec<i8>>> = Vec::new();
//         $( 
//             match $notes {
//                 Rest => {midi_values.push(None)},
//                 _ => {midi_values.push(Some($notes.play()));}
//             }
//         )*
//         midi_values
//     }}
// }


// /// This macro is used to conveniently define chords 
// macro_rules! chord {

// }

// TODO: play! macro: play![a, b, c] => vec![a.play(), b.play(), c.play()]