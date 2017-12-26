/// This file defines unit tests for DAWPL's various components
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
pub mod super_collider_templates;
#[macro_use]
pub mod syntax;
pub mod arrangement;
pub mod chord;
pub mod clip;
pub mod note;
pub mod scale;
pub mod track;
pub mod translate;



#[cfg(test)]
mod tests {
    use arrangement::*;
    use chord::*;
    use clip::*;
    use note::*;
    use scale::*;
    use syntax::*;
    use track::*;
    use translate::*;
    use super_collider_templates::*;

    #[test]
    fn note_tests() {
        let middle_c = Note::new(Name::C, 4);
        let b_natural = middle_c.flat();
        let d_natural = Note::new(Name::D, 4);
        let e_flat = d_natural.sharp();
        let e_natural = Note::new(Name::E, 4);
        assert_eq!(e_flat, e_natural.flat());
        assert_eq!(b_natural, d_natural.flat().flat().flat());
    }

    #[test]
    fn scale_tests() {
        let c_major = Scale::new(Note::new(Name::C, 4), ScaleType::Major);
        assert_eq!(c_major.get_degree(2), Note::new(Name::E, 4));
        assert_eq!(c_major.get_degree(3), Note::new(Name::F, 4));
        assert_eq!(c_major.get_degree(4), Note::new(Name::G, 4));
        assert_eq!(c_major.get_degree(5), Note::new(Name::A, 4));
        assert_eq!(c_major.get_degree(0), Note::new(Name::C, 4));
        assert_eq!(c_major.get_degree(-1), Note::new(Name::B, 3));
        assert_eq!(c_major.get_degree(7), Note::new(Name::C, 5));
        assert_eq!(c_major.get_degree(8), Note::new(Name::D, 5));
        // TODO: add tests for voicings and Arabic nubmers
    }

    #[test]
    fn chord_tests() {
        let c_maj7 = Chord::new(Note::new(Name::C, 4), ChordType::Maj7);
        let c_maj7_notes = c_maj7.get_voicing(0);
        assert_eq!(c_maj7_notes.len(), 4);
        assert_eq!(c_maj7_notes[0], Note::new(Name::C, 4));
        assert_eq!(c_maj7_notes[1], Note::new(Name::E, 4));
        assert_eq!(c_maj7_notes[2], Note::new(Name::G, 4));
        assert_eq!(c_maj7_notes[3], Note::new(Name::B, 4));
    }

    #[test]
    fn clip_translation_tests() { // TODO: add config files for tests
        let ii_chord = Chord::new(Note::new(Name::D, 4), ChordType::Min7);
        let ii_chord_clip = Clip::Instrument("v1".into(), "sine".into(), 
            vec![Some(ii_chord.play()), Some(ii_chord.play())],
            vec![1.0, 1.0]);
        let expected_ii =
"
v1 = Pbind(
    \\instrument, \\sine,
    \\dur, Pseq([1, 1]),
    \\midinote, Pseq([[62, 65, 69, 72],[62, 65, 69, 72],]),
);
";
        let clip_output = clip_to_super_collider(&ii_chord_clip);
        //println!("Clip output: {}", clip_output);
        //println!("Expected output: {}", expected_ii);
        assert_eq!(expected_ii, clip_output);

    }

    #[test]
    fn track_translation_tests() {
        let t = Track::new("t1".into(), vec!["v1".into(), "v2".into()]);
        //println!("Track output: {}", track_to_super_collider(&t));
    }

    #[test]
    fn arrangement_translation_tests() {
        let ii_chord = Chord::new(n!(D4), ChordType::Min7);
        let ii_chord_clip = Clip::Instrument("v1".into(), "sine".into(), 
            vec![Some(ii_chord.play()), Some(ii_chord.play())],
            vec![1.0, 1.0]);
        let V_chord = Chord::new(Note::new(Name::G, 3), ChordType::Dom7);
        let V_chord_clip = Clip::Instrument("v2".into(), "sine".into(),
            vec![Some(V_chord.play()), Some(V_chord.play())],
            vec![1.0, 1.0]);
        let t = Track::new("t1".into(), vec!["v1".into(), "v2".into()]);
        let arr: Arrangement = Arrangement::new(vec![t],
            vec![ii_chord_clip, V_chord_clip]);
        //println!("Arrangement output: {}", arrangement_to_super_collider(&arr));
        
    }

    #[test]
    fn syntax_tests() {
        assert_eq!(n!(C4), Note::new(Name::C, 4));
        assert_eq!(chord!(C4, Maj7), Chord::new(Note::new(Name::C, 4),
            ChordType::Maj7));
        assert_eq!(rhythm![W, Q, W, H], vec![1.0, 0.25, 1.0, 0.5]);
        println!("track: {:?}", track!(track1, clip1, clip2));
        assert_eq!(track!(track1, clip1, clip2), Track::new("track1".into(),
            vec!["clip1".into(), "clip2".into()]));
        println!("Play output: {:?}", play!(n!(C4), chord!(C4, Maj7), ()));
        assert_eq!(play!(n!(C4), chord!(C4, Maj7), ()),
            vec![Some(vec![60]), Some(vec![60, 64, 67, 71]), None]);
    }

    #[test]
    fn full_test() {
        let ii_chord = chord!(D4, Min7).play();
        let V_chord = chord!(G3, Dom7).play();
        let I_chord = chord!(C4, Maj7).play();
        let mut melody_notes: Vec<i8> = Vec::new();
        melody_notes.extend(&ii_chord);
        melody_notes.extend(&V_chord);
        melody_notes.extend(&I_chord);
        let progression_clip = instr_clip!(prog, sine,
            vec![Some(ii_chord), Some(V_chord), Some(I_chord)], rhythm![W, W, W]);
        let melody_clip = instr_clip!(melody, sine,
            melody_notes.into_iter().map(|n| Some(vec![n])).collect(),
            rhythm![Q, Q, Q, Q, Q, Q, Q, Q, Q, Q, Q, Q]);
        let arr: Arrangement = Arrangement::new(
            vec![track!(progTrack, prog), track!(melTrack, melody)],
            vec![progression_clip, melody_clip]);
        println!("Arrangement output: {}", arrangement_to_super_collider(&arr));
    }
}
