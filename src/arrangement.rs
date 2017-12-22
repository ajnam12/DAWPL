// This file defines the arrangement interface.
use chord::*;
use clip::*;
use note::*;
use scale::*;
use syntax::*;
use track::*;
use translate::*;



pub struct Arrangement {
    /// All tracks that comprise the arrangement
    tracks: Vec<Track>,
    /// All clips used within a track
    clips: Vec<Clip>,
    // TODO: add volumes/combination effects
}

impl Arrangement {
    pub fn new(tracks: Vec<Track>, clips: Vec<Clip>) -> Arrangement {
        Arrangement {tracks, clips}
    }

    // Obtain a vector of all the names of tracks and clips
    pub fn get_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for clip in self.clips.iter() {
            names.push(clip.get_name().clone());
        }
        for track in self.tracks.iter() {
            names.push(track.get_name().clone());
        }
        names
    }

    pub fn get_clips_ref(&self) -> &Vec<Clip> {
        &self.clips
    }

    pub fn get_tracks_ref(&self) -> &Vec<Track> {
        &self.tracks
    }
}

