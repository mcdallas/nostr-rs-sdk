// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

use nostr::key::{FromBech32, Keys};
use nostr::util::nips::nip04::decrypt;
use nostr::util::time::timestamp;
use nostr::{Entity, Kind, KindBase, Sha256Hash, SubscriptionFilter};
use nostr_sdk::{Client, RelayPoolNotifications, Result};

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let my_keys = Keys::from_bech32(BECH32_SK)?;

    let proxy = Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050)));

    let client = Client::new(&my_keys);
    client.add_relay("wss://relay.nostr.info", proxy).await?;
    client.add_relay("wss://rsslay.fiatjaf.com", None).await?;
    client.add_relay("wss://relay.damus.io", None).await?;
    client.add_relay("wss://nostr.openchain.fr", None).await?;
    client
        .add_relay(
            "ws://jgqaglhautb4k6e6i2g34jakxiemqp6z4wynlirltuukgkft2xuglmqd.onion",
            proxy,
        )
        .await?;

    client.connect().await?;

    client
        .delete_event(
            Sha256Hash::from_str(
                "57689882a98ac4db67933196c121489dea7e1231f7c0f20accad4de838500edc",
            )?,
            Some("reason"),
        )
        .await?;

    let entity: Entity = client
        .get_entity_of("25e5c82273a271cb1a840d0060391a0bf4965cafeb029d5ab55350b418953fbb")
        .await?;
    println!("Entity: {:?}", entity);

    let subscription = SubscriptionFilter::new()
        .pubkey(my_keys.public_key())
        .since(timestamp());

    client.subscribe(vec![subscription]).await?;

    loop {
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotifications::ReceivedEvent(event) = notification {
                if event.kind == Kind::Base(KindBase::EncryptedDirectMessage) {
                    if let Ok(msg) = decrypt(&my_keys.secret_key()?, &event.pubkey, &event.content)
                    {
                        println!("New DM: {}", msg);
                    } else {
                        log::error!("Impossible to decrypt direct message");
                    }
                } else {
                    println!("{:?}", event);
                }
            }
        }
    }
}
