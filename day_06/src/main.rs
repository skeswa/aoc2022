extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod data_stream;

use anyhow::{Context, Result};
use data_stream::DataStream;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_data_stream = advent::data(&config).await?;

    let data_stream = DataStream::parse(&encoded_data_stream);

    match config.part {
        1 => {
            let start_of_packet_index = data_stream
                .start_of_packet_index()
                .context("Encoded data stream did not have a start of packet marker")?;

            println!("Start of packet index: {}", start_of_packet_index);
        }
        _ => {
            let start_of_message_index = data_stream
                .start_of_message_index()
                .context("Encoded data stream did not have a start of message marker")?;

            println!("Start of message index: {}", start_of_message_index);
        }
    }

    Ok(())
}
