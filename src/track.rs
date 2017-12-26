/// Defines the track abstraction. Tracks are sequences of clips. Rests can
/// be represented by inserting Empty clips into the track for the desired
/// duration.
/// Note that in their current state, Tracks are defined in terms of symbolic
/// relationships to clips i.e. they deal only with the names of clips, rather
/// than references to actual clip objects. In the future, clip_names might
/// later be changed to contain shared references to Clip structs OR there might
/// be another syntax-check to ensure that all the clip names used exist.
use arrangement::*;
use chord::*;
use clip::*;
use note::*;
use scale::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Track {
    name: String,
    clip_names: Vec<String> // Gaps can be represented as empty clips
}

impl Track {
    pub fn new(name: String, clip_names: Vec<String>) -> Track {
        Track {name, clip_names}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_names_ref(&self) -> &Vec<String> {
        &self.clip_names
    }

    pub fn append_clip(&mut self, clip_name: String) {
        self.clip_names.push(clip_name);
    }


}