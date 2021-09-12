fn main() {
    let rt = tokio::runtime::Runtime::new().expect("could not initialise runtime");
    let f = rt.spawn(async { wifiscanner::scan().await });

    let networks = rt.block_on(f).expect("cannot scan network");
    for network in networks.unwrap_or_default() {
        println!(
            "{} {:15} {:10} {:4} {}",
            network.mac, network.ssid, network.channel, network.signal_level, network.security
        );
    }
}
