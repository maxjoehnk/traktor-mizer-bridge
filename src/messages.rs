use crate::models::*;

#[derive(Debug, Clone)]
pub enum Messages {
    DeckLoaded(Deck, DeckLoaded),
    DeckUpdated(Deck, UpdateDeck),
    MasterClock(MasterClock),
}
