#![no_main]
use async_trait::async_trait;
#[path = "./region.rs"]
pub mod region;
#[async_trait]
pub trait Scraping {
    fn new(data: &region::Region) -> Self;
    async fn run(&self) -> anyhow::Result<Vec<String>>;
}
