use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let my_keys = Keys::parse("KEY HERE")?;

    // Show bech32 public key
    let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
    println!("Bech32 PubKey: {}", bech32_pubkey);

    // Create new client
    let client = Client::new(&my_keys);

    let proxy = Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050)));

    // Add relays
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay_with_opts(
        "wss://relay.nostr.info", 
        RelayOptions::new().proxy(proxy).write(false)
    ).await?;
    client.add_relay_with_opts(
        "ws://jgqaglhautb4k6e6i2g34jakxiemqp6z4wynlirltuukgkft2xuglmqd.onion",
        RelayOptions::new().proxy(proxy),
    ).await?;

    // Connect to relays
    client.connect().await;

    // Publish a text note
    client.publish_text_note("Testing", []).await?;

    Ok(())
}
