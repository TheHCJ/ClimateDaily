use std::{net::{Ipv4Addr, SocketAddr, SocketAddrV4}, sync::Arc, time::UNIX_EPOCH};
use http::{Request, Response};
use std::time::SystemTime;
use nostr_sdk::prelude::*;

fn calculate_temp_rise(t: u64) -> f64 {
    // Coefficients for calculating temperature rise
    let a: f64 = 1.2163;
    let b: f64 = 0.00000000075481;

    // Calculate temperature rise
    let temp_rise = a + (b * t as f64);

    temp_rise
}

fn calculate_carbon_emissions(start_time: u64, end_time: u64) -> f64 {
    let emissions_per_year = 44.0; // Carbon emissions per year in billion tonnes
    let seconds_per_year: f64 = (365 * 24 * 60 * 60) as f64; // Number of seconds in a year
    
    let duration_in_seconds = end_time - start_time;
    let total_emissions = duration_in_seconds as f64 * emissions_per_year / seconds_per_year;

    total_emissions
}

#[tokio::main]
async fn main() -> Result<()> {
    let my_keys = Keys::parse("")?;

    let mut get = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "my-awesome-agent/1.0");


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

    let basis_date = 1607980800;
    let system_time: SystemTime = SystemTime::now();
    let current_time = system_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let time_difference = current_time - basis_date;

    let temp_rise = calculate_temp_rise(time_difference);
    let carbon_emissions = calculate_carbon_emissions(time_difference, current_time);

    let mut note: String = String::new(); // Define note as mutable

    note.push_str("🔥 Today's global warming statistics 🔥");

    note.push_str("\n"); // newline

    note.push_str("Human-induced warming: ");
    note.push_str(&temp_rise.to_string());
    note.push_str("°C");

    note.push_str("\n"); // newline

    note.push_str("Carbon emissions: +");
    note.push_str(&carbon_emissions.to_string());
    note.push_str(" trillion tonnes");

    note.push_str("\n"); // newline
    
    println!("{}", note);
    

    // Publish a text note
    //client.publish_text_note(note, []).await?;

    Ok(())
}
