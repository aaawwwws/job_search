mod bin;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let region = bin::indeed::region::Region::new("北海道");
    let indeed = bin::indeed::Indeed::new(&region);
    let test = indeed.run().await?;
    for i in &test {
        print!("{}::{}", i.get_title(), i.get_url());
    }
    Ok(())
}
