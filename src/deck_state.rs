use std::time::Duration;
use serde_derive::Serialize;
use crate::models::{DeckLoaded, UpdateDeck};

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeckState {
    pub track: Option<Track>,
    pub playing: bool,
    pub tempo: f64,
    pub elapsed_time: Duration,
    pub master: bool,
}

impl DeckState {
    pub fn update(&mut self, update_deck: UpdateDeck) {
        if let Some(is_playing) = update_deck.is_playing {
            self.playing = is_playing;
        }
        if let Some(elapsed_time) = update_deck.elapsed_time {
            self.elapsed_time = Duration::from_secs_f64(elapsed_time);
        }
        if let Some(tempo) = update_deck.tempo {
            if let Some(bpm) = self.track.as_ref().map(|t| t.bpm) {
                self.tempo = bpm * tempo;
            }
        }
    }

    pub fn update_master(&mut self, master: bool) {
        self.master = master;
    }

    pub fn load(&mut self, deck_loaded: DeckLoaded) {
        self.playing = deck_loaded.is_playing;
        self.tempo = deck_loaded.bpm * deck_loaded.tempo;
        self.track = Some(deck_loaded.into());
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub length: Duration,
    pub bpm: f64,
}

impl From<DeckLoaded> for Track {
    fn from(value: DeckLoaded) -> Self {
        Self {
            title: value.title,
            album: value.album,
            artist: value.artist,
            length: Duration::from_secs_f64(value.track_length),
            bpm: value.bpm,
        }
    }
}
