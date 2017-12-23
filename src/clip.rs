/// This file defines DAWPL clips and the various methods they support for 
/// editing and translation. Clips can also be extended to support effects
/// on the signals they contain.
/// Rest syntax: http://doc.sccode.org/Classes/Rest.html
use chord::*;
use note::*;
use scale::*;

pub enum Clip {
    // Box is used for dynamic dispatch purposes
    /// Clip name, instrument name, notes, durations
    Instrument(String, String, Vec<Option<Vec<i8>>>, Vec<f64>),
    /// Clip name, full path to audio file
    File(String, String),
    /// Clip name, duration of the rest
    Empty(String, f64),
}

impl Clip {
    pub fn get_name(&self) -> String {
        match self {
            &Clip::Instrument(ref name, _, _, _) => name.clone(),
            &Clip::File(ref name, _) => name.clone(),
            &Clip::Empty(ref name, _) => name.clone(),
        }
    }
}