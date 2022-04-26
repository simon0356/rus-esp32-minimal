use esp_idf_hal::prelude::*;
use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    loop {
        println!("Hello world");
        thread::sleep(Duration::from_secs(1));
    }
}
