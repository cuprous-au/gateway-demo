use std::{fmt::Display, path::PathBuf, time::Duration};

#[cfg(target_os = "linux")]
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use tokio::sync::broadcast;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Opened,
    Closed,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Opened => f.write_str("Tamper opened"),
            Event::Closed => f.write_str("Tamper closed"),
        }
    }
}

const SAMPLING_DELAY: Duration = Duration::from_millis(100);

#[cfg(target_os = "linux")]
fn detect_event(tamper_rx: &LineHandle) -> Result<Event, gpio_cdev::Error> {
    let state = tamper_rx.get_value()?;
    let result = if state == 1 {
        Event::Opened
    } else {
        Event::Closed // Active low
    };
    Ok(result)
}

// The Cuprux target
#[cfg(target_os = "linux")]
pub async fn task(
    event_tx: broadcast::Sender<Event>,
    tamper_rx_chip_path: PathBuf,
    tamper_rx_line: u32,
) -> Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new(tamper_rx_chip_path)?;
    let line = chip.get_line(tamper_rx_line)?;
    let tamper_rx = line.request(LineRequestFlags::INPUT, 0, "tamper")?;
    let event = detect_event(&tamper_rx)?;
    let _ = event_tx.send(event.clone());
    let mut last_event = event;
    loop {
        tokio::time::sleep(SAMPLING_DELAY).await;

        if event_tx.receiver_count() == 0 {
            break;
        }

        let event = detect_event(&tamper_rx)?;
        if event != last_event {
            if event_tx.send(event.clone()).is_err() {
                break;
            }
            last_event = event;
        }
    }

    Ok(())
}

// Development mode target
#[cfg(not(target_os = "linux"))]
pub async fn task(
    event_tx: broadcast::Sender<Event>,
    _tamper_rx_chip_path: PathBuf,
    _tamper_rx_line: u32,
) -> Result<(), gpio_cdev::Error> {
    let mut count = 0u32;

    loop {
        tokio::time::sleep(SAMPLING_DELAY * 10).await;

        if event_tx
            .send(if count % 2 == 0 {
                Event::Closed
            } else {
                Event::Opened
            })
            .is_err()
        {
            break;
        }

        count = count.wrapping_add(1);
    }

    Ok(())
}
