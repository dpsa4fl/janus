use janus_aggregator::{binaries::aggregator::main_callback, binary_utils::janus_main};
use janus_core::time::RealClock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    janus_main(RealClock::default(), main_callback).await
}
