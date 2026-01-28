use std::sync::mpsc;

use color_eyre::eyre::Context;
use log::LevelFilter;
use zeevonk::client::Client;
use zeevonk::ident::Identifier;
use zeevonk::trigger::{Trigger, TriggerValue};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let is_debug_mode = cfg!(debug_assertions);
    let default_level = if is_debug_mode { LevelFilter::Debug } else { LevelFilter::Info };
    pretty_env_logger::formatted_builder().filter_level(default_level).parse_env("RUST_LOG").init();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    bmdse::SpeedEditor::new()
        .context("failed to open Speed Editor device")?
        .on_button_change({
            let event_tx = mpsc::Sender::clone(&event_tx);
            move |button, on| {
                event_tx.send(Event::Button { button, on }).expect("channel should be open");
            }
        })
        .on_wheel_change({
            let event_tx = mpsc::Sender::clone(&event_tx);
            move |velocity| {
                event_tx.send(Event::Wheel { velocity }).expect("channel should be open");
            }
        });

    let mut client = Client::new(Identifier::new("zv-ctrl-bmdse").unwrap());
    client.connect("ws://127.0.0.1:7334").await.context("failed to connect client to server")?;

    loop {
        match event_rx.recv().expect("channel should be open") {
            Event::Button { button, on } => {
                let id = id_from_button(button);
                let value = TriggerValue::Boolean(on);

                let _ = client
                    .send_trigger(Trigger::new(id, value))
                    .await
                    .map_err(|err| log::error!("could not send trigger: {err}"));
            }
            Event::Wheel { velocity } => {
                let id = Identifier::new("wheel-velocity").unwrap();
                let value = TriggerValue::Integer(velocity as i64);

                let _ = client
                    .send_trigger(Trigger::new(id, value))
                    .await
                    .map_err(|err| log::error!("could not send trigger: {err}"));
            }
        }
    }
}

enum Event {
    Button { button: bmdse::Button, on: bool },
    Wheel { velocity: i32 },
}

fn id_from_button(button: bmdse::Button) -> Identifier {
    let raw = match button {
        bmdse::Button::SmartInsert => "button-smart-insert",
        bmdse::Button::Append => "button-append",
        bmdse::Button::RippleOverwrite => "button-ripple-overwrite",
        bmdse::Button::CloseUp => "button-close-up",
        bmdse::Button::PlaceOnTop => "button-place-on-top",
        bmdse::Button::SourceOverwrite => "button-source-overwrite",
        bmdse::Button::In => "button-in",
        bmdse::Button::Out => "button-out",
        bmdse::Button::TrimIn => "button-trim-in",
        bmdse::Button::TrimOut => "button-trim-out",
        bmdse::Button::Roll => "button-roll",
        bmdse::Button::SlipSource => "button-slip-source",
        bmdse::Button::SlipDestination => "button-slip-destination",
        bmdse::Button::TransitionDuration => "button-transition-duration",
        bmdse::Button::Cut => "button-cut",
        bmdse::Button::Dissolve => "button-dissolve",
        bmdse::Button::SmoothCut => "button-smooth-cut",
        bmdse::Button::Escape => "button-escape",
        bmdse::Button::SyncBin => "button-sync-bin",
        bmdse::Button::AudioLevel => "button-audio-level",
        bmdse::Button::FullView => "button-full-view",
        bmdse::Button::Transition => "button-transition",
        bmdse::Button::Split => "button-split",
        bmdse::Button::Snap => "button-snap",
        bmdse::Button::RippleDelete => "button-ripple-delete",
        bmdse::Button::Cam1 => "button-cam-1",
        bmdse::Button::Cam2 => "button-cam-2",
        bmdse::Button::Cam3 => "button-cam-3",
        bmdse::Button::Cam4 => "button-cam-4",
        bmdse::Button::Cam5 => "button-cam-5",
        bmdse::Button::Cam6 => "button-cam-6",
        bmdse::Button::Cam7 => "button-cam-7",
        bmdse::Button::Cam8 => "button-cam-8",
        bmdse::Button::Cam9 => "button-cam-9",
        bmdse::Button::LiveOverwrite => "button-live-overwrite",
        bmdse::Button::VideoOnly => "button-video-only",
        bmdse::Button::AudioOnly => "button-audio-only",
        bmdse::Button::StopPlay => "button-stop-play",
        bmdse::Button::Source => "button-source",
        bmdse::Button::Timeline => "button-timeline",
        bmdse::Button::Shuttle => "button-shuttle",
        bmdse::Button::Jog => "button-jog",
        bmdse::Button::Scroll => "button-scroll",
    };

    Identifier::new(raw).unwrap()
}
