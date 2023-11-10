mod bin;
use anyhow;
use bin::stanb;
use polars::prelude::*;
use std::env;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let region = bin::stanb::region::Region::new("北海道 千歳市");
    let stanby = stanb::Stanb::new(&region);
    let result = stanby.run().await?;
    /*
    const MAX: u8 = 20;
    env::set_var("POLARS_FMT_MAX_COLS", "255");
    env::set_var("POLARS_FMT_MAX_ROWS", "255");
    env::set_var("POLARS_FMT_STR_LEN", MAX.to_string());
    let region = bin::indeed::region::Region::new("北海道 千歳市");
    let indeed = bin::indeed::Indeed::new(&region);
    let test = indeed.run().await?;
    let mut titles: Vec<String> = vec![];
    let mut com_name: Vec<String> = vec![];
    let mut location: Vec<String> = vec![];
    let mut amount: Vec<String> = vec![];
    let mut employment_status: Vec<String> = vec![];
    let mut description: Vec<String> = vec![];
    let mut urls: Vec<String> = vec![];
    for i in test.iter() {
        titles.push(i.get_title().to_string());
        com_name.push(i.get_com_name().to_string());
        location.push(i.get_location().to_string());
        amount.push(i.get_amount().to_string());
        employment_status.push(i.get_employment_status().to_string());
        description.push(i.get_description().to_string());
        urls.push(i.get_url().to_string());
    }

    let mut df = df!(
        "タイトル" => &titles,
        "企業名" => &com_name,
        "場所" => &location,
        "金額" => &amount,
        "雇用形態" => &employment_status,
        "説明" => &description,
        "URL" => &urls,
    )?;
    let path = String::from("./test.csv");
    let mut test = CsvWriter::new(std::fs::File::create(&path)?);
    let ok = test.finish(&mut df);
    print!("{}", df);
     */
    Ok(())
}
