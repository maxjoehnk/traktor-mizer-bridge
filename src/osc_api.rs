use std::time::SystemTime;
use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};
use tokio::net::UdpSocket;
use crate::config::OscConfig;
use crate::deck_state::DeckState;
use crate::DecksState;
use crate::models::Deck;

pub async fn start(config: OscConfig) -> color_eyre::Result<OscApi> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    Ok(OscApi {
        config,
        socket
    })
}

pub struct OscApi {
    config: OscConfig,
    socket: UdpSocket,
}

impl OscApi {
    pub async fn update(&self, state: DecksState) -> color_eyre::Result<()> {
        let mut builder = BundleBuilder::default();

        for (deck, state) in state.decks {
            let bundle = self.update_deck(deck, state)?;
            builder.add_bundle(bundle);
        }

        self.send_msg(builder.build()?).await?;

        Ok(())
    }

    fn update_deck(&self, deck: Deck, state: DeckState) -> color_eyre::Result<OscBundle> {
        let mut builder = BundleBuilder::new(format!("/decks/{deck}"));
        if let Some(track) = state.track {
            builder.add("title", track.title);
            builder.add("bpm", track.bpm);
            builder.add("length", track.length.as_secs_f64());
        }
        builder.add("tempo", state.tempo);
        builder.add("master", state.master);
        builder.add("playing", state.playing);
        builder.add("elapsed_time", state.elapsed_time.as_secs_f64());

        builder.build()
    }

    async fn send_msg(&self, bundle: OscBundle) -> color_eyre::Result<()> {
        let packet = rosc::encoder::encode(&OscPacket::Bundle(bundle))?;
        self.socket.send_to(&packet, (self.config.target, self.config.port)).await?;

        Ok(())
    }
}

#[derive(Default)]
struct BundleBuilder {
    packets: Vec<OscPacket>,
    prefix: Option<String>,
}

impl BundleBuilder {
    fn new(prefix: String) -> Self {
        Self {
            packets: Default::default(),
            prefix: Some(prefix),
        }
    }

    fn add_bundle(&mut self, bundle: OscBundle) {
        self.packets.push(OscPacket::Bundle(bundle));
    }

    fn add(&mut self, addr: &str, arg: impl Into<OscType>) {
        let msg = OscMessage {
            addr: format!("{prefix}/{addr}", prefix = self.prefix.as_deref().unwrap_or_default()),
            args: vec![arg.into()]
        };
        self.packets.push(OscPacket::Message(msg));
    }

    fn build(self) -> color_eyre::Result<OscBundle> {
        let bundle = OscBundle {
            timetag: OscTime::try_from(SystemTime::now())?,
            content: self.packets,
        };

        Ok(bundle)
    }
}
