use std::collections::HashMap;
use crate::config::Config;
use crate::deck_state::*;
use crate::messages::Messages;
use crate::models::Deck;

mod models;
mod http_api;
mod osc_api;
mod messages;
mod deck_state;
mod config;

#[derive(Debug, Clone)]
struct DecksState {
    pub decks: HashMap<Deck, DeckState>
}

impl Default for DecksState {
    fn default() -> Self {
        let mut decks = HashMap::new();
        decks.insert(Deck::A, Default::default());
        decks.insert(Deck::B, Default::default());
        decks.insert(Deck::C, Default::default());
        decks.insert(Deck::D, Default::default());

        Self {
            decks
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let config = Config::read().await?;

    let mut deck_state = DecksState::default();

    let (deck_tx, mut deck_rx) = tokio::sync::mpsc::unbounded_channel();

    let http_handle = tokio::spawn(http_api::start(deck_tx, config.http));
    let osc_handle = if let Some(osc_config) = config.osc {
        Some(osc_api::start(osc_config).await?)
    }else {
        None
    };

    while let Some(msg) = deck_rx.recv().await {
        match msg {
            Messages::DeckLoaded(deck, loaded) => {
                if let Some(state) = deck_state.decks.get_mut(&deck) {
                    state.load(loaded);
                }
            }
            Messages::DeckUpdated(deck, update) => {
                if let Some(state) = deck_state.decks.get_mut(&deck) {
                    state.update(update);
                }
            }
            Messages::MasterClock(clock) => {
                for (deck, deck_state) in deck_state.decks.iter_mut() {
                    deck_state.update_master(clock.deck.as_ref() == Some(deck))
                }
            }
        }
        tracing::trace!("State: {deck_state:?}");
        if let Some(osc_api) = osc_handle.as_ref() {
            osc_api.update(deck_state.clone()).await?;
        }
    }

    http_handle.await??;

    Ok(())
}
