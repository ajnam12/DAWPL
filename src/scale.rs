/// Defines the scale abstraction. Scales are sequences of notes defined
/// relative to a base note and a formula of intervals. Here, we use a formula
/// of cumulative intervals to make some logic in the code slightly simpler.
use note::*;
use std::collections::HashMap;

lazy_static! {
    // A table mapping scale types to sequences of half-step off-sets from the 
    // root note. The sequences end right before the octave
    static ref SCALE_FORMULAS: HashMap<ScaleType, Vec<i8>> = {
        [(ScaleType::Major, vec![0, 2, 4, 5, 7, 9, 11]),
         (ScaleType::Mixolydian, vec![0, 2, 4, 5, 7, 9, 10]),
         (ScaleType::Dorian, vec![0, 2, 3, 5, 7, 9, 10])]
        .iter().cloned().collect()
    }; 
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum ScaleType {
    Major,
    Mixolydian,
    Dorian,
    Other // TODO: add more e.g. Blues, MajorPentatonic, MinorPentatonic
}

/// Arabic number tones are used in Jazz theory to convey the position of a 
/// They are often used relative to a major scale to describe formulas for
/// chords and scales in general.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum ArabicNum {
    Natural(i8),
    Sharp(i8),
    Flat(i8)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Scale {
    base_note: Note,
    scale_type: ScaleType
}

// TODO: add scale macro/function e.g. scale!("C4Maj") (forgo string?)
impl Scale {
    pub fn new(base_note: Note, scale_type: ScaleType) -> Scale {
        Scale {base_note, scale_type}
    }

    /// Returns the note that is degree positions away from the base note of
    /// the scale.
    /// # Example (TODO: add examples from lib tests)
    pub fn get_degree(&self, degree: i8) -> Note {
        // TODO: add bounds checking
        // First, account for multiple octave skips
        let scale = SCALE_FORMULAS.get(&self.scale_type).unwrap();
        let scale_len = scale.len() as i8;
        let octave_offset = degree / scale_len;
        let scale_offset = degree % scale_len;
        let half_step_offset = if scale_offset < 0 {
            scale[(scale_offset + scale_len) as usize] - (NUM_TONES as i8)
        } else {
            scale[scale_offset as usize]
        };
        self.base_note.add_half_steps(octave_offset * (NUM_TONES as i8) +
            half_step_offset)

    }

    /// Returns a note with the associated arabic num, which comes after
    /// preceeding_note. This is convenient for describing voicings.
    /// The preceeding_note must be in the octave of the base_note or the
    /// octave afterward.
    pub fn get_arabic_num(&self, arabic_num: &ArabicNum, preceeding_note: &Note) 
        -> Note {
        let mut result_note = match *arabic_num {
            ArabicNum::Natural(num) => self.get_degree(num - 1),
            ArabicNum::Flat(num) => self.get_degree(num - 1).flat(),
            ArabicNum::Sharp(num) => self.get_degree(num - 1).sharp(),
        };
        result_note = Note::new(result_note.get_name(),
            preceeding_note.get_octave());
        if result_note < *preceeding_note {
            Note::new(result_note.get_name(), result_note.get_octave() + 1)
        } else {
            result_note
        }
    }

    /// Given a vector of arabic numerals, return a vector of notes from the
    /// scale that correspond to those numbers. The first arabic number in the
    /// voicing vector corresponds to the tone with that arabic number relative
    /// to the base_note. The numbers afterward describe the next notes in
    /// ascending order (in order to have consistency and a unique, unambiguous
    /// vector of notes to correspond to the voicing)
    pub fn get_voicing(&self, voicing: &Vec<ArabicNum>) -> Vec<Note> {
        let mut voicing_notes: Vec<Note> = Vec::new();
        for num in voicing.iter() {
            let last_note = if voicing_notes.len() == 0 {
                self.base_note
            } else {
                voicing_notes[voicing_notes.len() - 1]
            };
            voicing_notes.push(self.get_arabic_num(&num, &last_note));
        }
        voicing_notes
    }
    // TODO: implement get_arabic_num
}