use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedSender;
use crate::config::HttpSourceConfig;
use crate::messages::Messages;
use crate::models::*;

#[derive(Clone)]
struct ApiState {
    tx: Arc<UnboundedSender<Messages>>,
}

impl ApiState {
    fn new(tx: UnboundedSender<Messages>) -> Self {
        Self {
            tx: Arc::new(tx),
        }
    }
}

pub async fn start(tx: UnboundedSender<Messages>, config: HttpSourceConfig) -> color_eyre::Result<()> {
    let router = Router::new()
        .route("/deckLoaded/:deck", post(deck_loaded))
        .route("/updateDeck/:deck", post(update_deck))
        .route("/updateMasterClock", post(update_master_clock))
        .route("/updateChannel/:channel", post(update_channel))
        .with_state(Arc::new(ApiState::new(tx)));

    let listener = TcpListener::bind((config.address, config.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn deck_loaded(State(state): State<Arc<ApiState>>, Path(deck): Path<Deck>, Json(body): Json<DeckLoaded>) -> StatusCode {
    tracing::debug!("RECV: deck_loaded {deck}: {body:?}");
    if let Err(err) = state.tx.send(Messages::DeckLoaded(deck, body)) {
        tracing::error!("Unable to send internal deck loaded message: {err:?}");
    }

    StatusCode::NO_CONTENT
}

async fn update_deck(State(state): State<Arc<ApiState>>, Path(deck): Path<Deck>, Json(body): Json<UpdateDeck>) -> StatusCode {
    tracing::debug!("RECV: update_deck {deck}: {body:?}");
    if let Err(err) = state.tx.send(Messages::DeckUpdated(deck, body)) {
        tracing::error!("Unable to send internal deck loaded message: {err:?}");
    }

    StatusCode::NO_CONTENT
}

async fn update_master_clock(State(state): State<Arc<ApiState>>, Json(body): Json<MasterClock>) -> StatusCode {
    tracing::debug!("RECV: update_master_clock: {body:?}");
    if let Err(err) = state.tx.send(Messages::MasterClock(body)) {
        tracing::error!("Unable to send internal deck loaded message: {err:?}");
    }

    StatusCode::NO_CONTENT
}

async fn update_channel(Path(channel): Path<u8>, Json(body): Json<UpdateChannel>) -> StatusCode {
    tracing::debug!("RECV: update_channel {channel}: {body:?}");

    StatusCode::NO_CONTENT
}
