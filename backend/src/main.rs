use std::{error::Error, net::SocketAddr, path::PathBuf};

use clap::Parser;
use log::info;
use tokio::sync::broadcast;

mod http_server;
mod tamper_switch;

/// A simple demo to serve a web app and http based events in response to the
/// Cuprous Secured Edge Gateway's tamper button being pressed.
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None, version)]
struct Args {
    /// The path to the SPA assets
    #[clap(env, long, default_value = "/lib/gateway-demo")]
    assets_path: PathBuf,

    /// A socket address for serving our web service requests.
    /// Defaults to the local interface.
    #[clap(env, long, default_value = "127.0.0.1:8081")]
    http_addr: SocketAddr,

    /// The GPIO chip to use that hosts the rx tamper switch pin
    #[clap(env, long, default_value = "/dev/gpiochip0")]
    tamper_rx_chip_path: PathBuf,

    /// The GPIO pin to use for the rx tamper switch pin
    #[clap(env, long, default_value_t = 14)]
    tamper_rx_pin: u32,
}

const MAX_TAMPER_EVENTS: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::builder().format_timestamp_millis().init();

    // Start up the tamper switch detection

    let (tamper_event_tx, _tamper_event_rx) = broadcast::channel(MAX_TAMPER_EVENTS);
    let tamper_switch_task = tokio::spawn(tamper_switch::task(
        tamper_event_tx.clone(),
        args.tamper_rx_chip_path,
        args.tamper_rx_pin,
    ));

    // Start up the http service

    let http_addr = args.http_addr;
    let http_server_task = tokio::spawn(async move {
        let routes = http_server::routes(&args.assets_path, tamper_event_tx);
        let listener = tokio::net::TcpListener::bind(http_addr).await?;
        info!("HTTP listening on {}", args.http_addr);
        let r = axum::serve(listener, routes).await;
        println!("finished");
        r
    });

    info!("gateway demo ready");

    tokio::select! {
        r = tamper_switch_task =>
        r.map_err(|e| e.into())
            .and_then(|r| r.map_err(|e| e.into())),

        r = http_server_task =>
        r.map_err(|e| e.into())
            .and_then(|r| r.map_err(|e| e.into())),
    }
}
