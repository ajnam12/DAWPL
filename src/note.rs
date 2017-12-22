/// Defines the note abstraction, which serves as a basis for producing sounds
/// from an instrument. Notes are also important for developing and defining
/// other abstractions such as scales and chords.
use std::collections::HashMap;
use std::cmp::Ordering;
use std::str::FromStr;

pub const MIDI_NUM: u8 = 128; // number of possible MIDI note values
pub const NUM_TONES: u8 = 12;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Note {
    name: Name,
    octave: i8,
}

lazy_static! {
    // mappings from midi numbers to (tone, octave) (and the reverse direction)
    static ref MIDI_MAPS: (Vec<Note>, HashMap<Note, i8>) = {
        let notes: Vec<Name> = vec![Name::C, Name::Db, Name::D, Name::Eb, 
        Name::E, Name::F, Name::Gb, Name::G, Name::Ab, Name::A, Name::Bb, 
        Name::B];
        let mut midi_to_note: Vec<Note> = Vec::new();
        let mut note_to_midi: HashMap<Note, i8> = HashMap::new();
        for i in 0..MIDI_NUM {
            // octave adjusted so middle C (midi num 60) is C4
            // in order to be consistent with scientific pitch notation
            let note_name: Name = notes[(i % NUM_TONES) as usize];
            let note_octave: i8 = ((i / NUM_TONES) as i8) - 1;
            let new_note = Note {name: note_name, octave: note_octave};
            midi_to_note.push(new_note);
            note_to_midi.insert(new_note, i as i8);
        }
        (midi_to_note, note_to_midi)
    };
}

/// Names of notes, sharps are currently not included for sake of simplicity as
/// wella as eliminiting redundancy.
/// Most Jazz-theoretic formulas are based on flats e.g. Mixolydian scales are
/// just major scales with a flat 7.
// usage of custom_derive inspired by
// https://stackoverflow.com/questions/39070244/
// can-i-convert-a-string-to-enum-without-macros-in-rust
custom_derive! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, EnumFromStr)]
    pub enum Name {C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B}
}

/// Whole note, half note, quarter note, eighth note, sixteenth note
custom_derive! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, EnumFromStr)]
    pub enum Beat {W, H, Q, E, S}
}


pub trait Playable {
    fn play(&self) -> Vec<i8>;
}

impl Playable for Note {
    fn play(&self) -> Vec<i8> {
        vec![self.get_midi_value()]
    }
}

/// When multiple notes are in a vector, they will be sounded at the same time
/// as one unit.
impl Playable for Vec<Note> {
    fn play(&self) -> Vec<i8> {
        self.iter()
        .map(|note: &Note| note.get_midi_value())
        .collect()
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Note) -> Option<Ordering> {
        Some(self.get_midi_value().cmp(&other.get_midi_value()))
    }
}

// TODO: add macro for creating notes e.g. note!("C4")
impl Note {
    pub fn new(name: Name, octave: i8) -> Note {
        Note {name, octave}
    }

    pub fn get_name(&self) -> Name {
        self.name
    }

    pub fn get_octave(&self) -> i8 {
        self.octave
    }

    /// Returns the note that is one half step above this one
    pub fn sharp(&self) -> Note {
        self.add_half_steps(1)
    }

    /// Returns the note that is one half step below this one
    pub fn flat(&self) -> Note {
        self.add_half_steps(-1)
    }

    /// Returns the MIDI value associated with this particular note
    pub fn get_midi_value(&self) -> i8 {
        let (_, ref note_to_midi) = *MIDI_MAPS;
        *(note_to_midi.get(&self).unwrap())
    }

    /// Adds a given number of half steps (potentially negative) to this note
    /// and returns that note.
    // TODO: add bounds checking (assertions should work)
    pub fn add_half_steps(&self, num_half_steps: i8) -> Note {
        let (ref midi_to_note, ref note_to_midi) = *MIDI_MAPS;
        let new_midi_num = note_to_midi.get(&self).unwrap() 
            + num_half_steps;
        midi_to_note[new_midi_num as usize]
    }

    /// Adds a certain number of whole steps to the given note
    pub fn add_whole_steps(&self, num_whole_steps: i8) -> Note {
        self.add_half_steps(num_whole_steps * 2)
    }
}

// Inspiration from this example:
// https://stackoverflow.com/questions/36508580/
// how-to-convert-a-string-to-an-enum
impl FromStr for Note {
    type Err = &'static str;
    fn from_str(note: &str) -> Result<Self, Self::Err> {
        assert!(note.len() > 0);
        let name_octave: Vec<char> = note.chars().collect();
        let name: Name = name_octave[0].to_string().parse().unwrap();
        match name_octave.len() {
            2 => {
                let octave: i8 = name_octave[1].to_string().parse().unwrap();
                Ok(Note::new(name, octave))
            },
            3 => {
                let octave: i8 = name_octave[2].to_string().parse().unwrap();
                match name_octave[1] {
                    's' => Ok(Note::new(name, octave).sharp()),
                    'b' => Ok(Note::new(name, octave).flat()),
                    _ => Err("Invalid input for note"),

                }
            },
            _ => Err("Invalid input for note"),
        }
    }
}


