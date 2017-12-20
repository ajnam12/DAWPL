
use arrangement::*;
use chord::*;
use clip::*;
use note::*;
use scale::*;

#[derive(Debug)]
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