/// Defines the chord abstraction. Chords provide a foundation for composition
/// and improvisation. They are often defined relative to scale tones. In the
/// CHORD_FORMULAS variable, we define chords relative to a sequence of Arabic
/// number tones of a major scale.
use note::*;
use scale::*;
use std::collections::HashMap;

custom_derive! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, EnumFromStr)]
    pub enum ChordType {Maj7, Min7, Dom7, Dim, Aug, Maj6}
}

lazy_static! {
    static ref CHORD_FORMULAS: HashMap<ChordType, Vec<ArabicNum>> = {
        // negative numbers will denote flat tones (relative to a major scale)
        [(ChordType::Maj7, vec![ArabicNum::Natural(1), ArabicNum::Natural(3),
            ArabicNum::Natural(5), ArabicNum::Natural(7)]),
         (ChordType::Min7, vec![ArabicNum::Natural(1), ArabicNum::Flat(3), 
            ArabicNum::Natural(5), ArabicNum::Flat(7)]),
         (ChordType::Dom7, vec![ArabicNum::Natural(1), ArabicNum::Natural(3), 
            ArabicNum::Natural(5), ArabicNum::Flat(7)]),
         (ChordType::Dim, vec![ArabicNum::Natural(1), ArabicNum::Flat(3),
            ArabicNum::Flat(5)]),
         (ChordType::Aug, vec![ArabicNum::Natural(1), ArabicNum::Natural(3),
            ArabicNum::Sharp(5)]),
         (ChordType::Maj6, vec![ArabicNum::Natural(1), ArabicNum::Natural(3),
            ArabicNum::Natural(5), ArabicNum::Natural(6)])]
         .iter().cloned().collect()
    };
}

// TODO: add chord! macro, e.g. chord!("C4maj7", 3, 7, 9)
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Chord {
    root: Note,
    chord_type: ChordType,
    ref_scale: Scale
}

impl Playable for Chord {
    fn play(&self) -> Vec<i8> {
        self.get_voicing(0).play()
    }
}

impl Chord {
    pub fn new(root: Note, chord_type: ChordType) -> Chord {
        // the reference scale is stored here so it can be used for voicings
        Chord {root, chord_type, ref_scale: Scale::new(root, ScaleType::Major)}
    }

    // TODO: rootless voicings in A/B positions?

    /// Given a starting note for the inversion of the chord (first_note) and 
    /// any alterations (e.g. flattened notes, sharped notes will be converted 
    /// to flattened notes internally for the sake of consistency)
    /// first_pos refers to the first position within the chord vector.
    /// We assume there are no redundant alterations
    // TODO: support alterations at this step? (can be done w/ scale/nums)
    pub fn get_voicing(&self, first_pos: usize) -> Vec<Note> {
        // TODO: Implement this, define this in terms of Scale's get voicing
        // define voicing vector to input into scale's voicing function
        let formula = CHORD_FORMULAS.get(&self.chord_type).unwrap();
        // re-arrange so that we obtain our desired inversion as specified
        // by first_pos (can do this with vector slices)
        let mut voicing: Vec<ArabicNum> = Vec::new();
        voicing.extend(&formula[first_pos..]);
        voicing.extend(&formula[..first_pos]);
        self.ref_scale.get_voicing(&voicing)

    }
}

// Given a starting note (as the root of the ii of a ii-V-I chord progression),
// return a vector containing the three chords with alternating inversions.
// pub fn two_five_one(base_note: Note) -> Vec<Chord> {
//     Vec::new()
// }