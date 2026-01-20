use zeevonk::client::controller::Client;
use zeevonk::ident::Identifier;
use zeevonk::trigger::{Trigger, TriggerValue};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut client = Client::new(Identifier::new("zv-ctrl-bmdse").unwrap());
    client.connect("ws://127.0.0.1:7335").await.unwrap();

    loop {
        client
            .send_trigger(Trigger::new(
                Identifier::new("button-1").unwrap(),
                TriggerValue::Boolean(true),
            ))
            .await
            .unwrap();

        std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 10.0));
    }
}
