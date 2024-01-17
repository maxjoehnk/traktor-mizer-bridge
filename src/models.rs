use std::fmt;
use serde_derive::Deserialize;
use serde_with::serde_as;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Hash)]
pub enum Deck {
    A,
    B,
    C,
    D
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterClock {
    pub bpm: f64,
    pub deck: Option<Deck>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDeck {
    pub elapsed_time: Option<f64>,
    pub next_cue_pos: Option<f64>,
    pub is_playing: Option<bool>,
    pub tempo: Option<f64>,
    pub is_synced: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckLoaded {
    pub title: String,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub album: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub artist: Option<String>,
    pub bpm: f64,
    pub is_playing: bool,
    pub tempo: f64,
    pub track_length: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChannel {
    pub is_on_air: bool,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::models::{Deck, DeckLoaded};

    #[test]
    fn parse_deck() -> Result<(), serde_json::Error> {
        let deck_a = serde_json::from_str::<Deck>("\"A\"")?;
        let deck_b = serde_json::from_str::<Deck>("\"B\"")?;
        let deck_c = serde_json::from_str::<Deck>("\"C\"")?;
        let deck_d = serde_json::from_str::<Deck>("\"D\"")?;

        assert_eq!(deck_a, Deck::A);
        assert_eq!(deck_b, Deck::B);
        assert_eq!(deck_c, Deck::C);
        assert_eq!(deck_d, Deck::D);
        Ok(())
    }

    #[test]
    fn parse_deck_loaded() -> Result<(), serde_json::Error> {
        let input = json!({ "title": "", "album": "", "artist": "", "bpm": 0, "isPlaying": false, "tempo": 0, "trackLength": 0 });

        let model = serde_json::from_value::<DeckLoaded>(input)?;

        assert_eq!(model.artist, None);
        assert_eq!(model.album, None);
        Ok(())
    }
}
