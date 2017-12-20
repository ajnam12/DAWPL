/// Translation module that compiles DAWPL to SuperCollider
use arrangement::*;
use chord::*;
use clip::*;
use note::*;
use scale::*;
use track::*;
use super_collider_templates::*;

pub fn arrangement_to_super_collider(arrangement: &Arrangement) -> String {
    // Handle appropriate variable declaration
    let mut var_decl: String = "var ".into();
    let names = arrangement.get_names();
    for i in 0..names.len() {
        var_decl += &(names[i])[..];
        if i < names.len() - 1 {
            var_decl += ",";
        }
    }
    var_decl += ";";
    // add clip declarations
    let mut clip_decl: String = "".into();
    for clip in arrangement.get_clips_ref().iter() {
        clip_decl += &(clip_to_super_collider(&clip))[..];
        clip_decl += "\n";
    }
    // add track declarations
    let mut track_decl: String = "".into();
    let mut track_name_str: String = "[".into(); // list of track names
    for track in arrangement.get_tracks_ref().iter() {
        track_decl += &(track_to_super_collider(&track))[..];
        track_decl += "\n";
        track_name_str += &format!("{},", track.get_name());
    }
    track_name_str += "]";
    // put declarations together to form arrangement
    format!(k_arrangement_template!(), instruments=k_instruments!(),
        variable_declarations=var_decl, clip_declarations=clip_decl,
        track_declarations=track_decl, track_names=track_name_str)
}

/// Utility function used for translating a vector of strings, which represent
/// variable names, to SuperCollider syntax for a list of those variables.
// pub fn translate_variable_list(var_list: &Vec<String>) -> String {
//     let mut vars_str: String = "[".into();
//     for name in var_list.iter() {
//         vars_str += &format!("{},", vars_str)[..];
//     }
//     vars_str += "]";
//     vars_str
// }

pub fn track_to_super_collider(track: &Track) -> String {
    //let clip_name_str = format!("{:?}", track.get_names_ref());
    let mut clip_name_str: String = "[".into();
    for clip_name in track.get_names_ref().iter() {
        clip_name_str += &format!("{},", clip_name)[..];
    }
    clip_name_str += "]";
    format!(k_track_template!(), track_name=track.get_name(),
        clips=clip_name_str)
}

/// This function translates a single clip to its SuperCollider equivalent.
/// Clip is the clip object itself, var is the name of the clip.
pub fn clip_to_super_collider(clip: &Clip) -> String {
    match clip {
        // Melody and durations must be of equivalent length
        &Clip::Instrument(ref var, ref name, ref melody, ref durations) => {
            assert_eq!(melody.len(), durations.len());
            let mut midi_note_str: String = "[".into();
            for i in 0..melody.len() {
                match melody[i] {
                    Some(ref notes) => {
                        let note_str = format!("{:?},", notes);
                        midi_note_str += &note_str[..];
                    },
                    None => {
                        midi_note_str += "note:Rest(),";
                    }
                }
            }
            midi_note_str += "]";
            format!(k_instrument_template!(), var_name=var, 
            instrument_name=name, dur=format!("{:?}", durations),
            midi_notes=midi_note_str)
        },
        &Clip::File(ref var, ref name) => {
            // TODO: fill this in
            "".into()
        },
        &Clip::Empty(ref var, ref duration) => {
            format!(k_empty_clip_template!(), dur=duration, var_name=var)
        }
    }
}
