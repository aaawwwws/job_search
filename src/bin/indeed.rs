use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
#[path = "./error.rs"]
mod error;
#[path = "./region.rs"]
pub mod region;
#[path = "./resultdata.rs"]
mod resultdata;

pub struct Indeed {
    url: String,
}

impl Indeed {
    pub fn new(data: &region::Region) -> Self {
        let search_url = String::from("https://jp.indeed.com/jobs?q=&l=");
        let u = format!("{}{}", search_url, data.get_region());
        return Self { url: u };
    }
    pub async fn run(&self) -> Result<Vec<resultdata::ResultData>, error::Errr> {
        let Ok((browser, mut handler)) = Browser::launch(
            BrowserConfig::builder()
                .window_size(1920, 1080)
                .with_head()
                .build()
                .unwrap(),
        )
        .await
        else {
            return Err(error::Errr::OpenError);
        };

        tokio::spawn(async move {
            loop {
                let _event = handler.next().await.unwrap();
            }
        });

        let Ok(page) = browser.new_page(&self.url).await else {
            return Err(error::Errr::PageError);
        };
        let Ok(elements) = page.find_elements("h2").await else {
            return Err(error::Errr::Null);
        };
        let mut datas: Vec<resultdata::ResultData> = Vec::new();
        for i in 0..elements.len() {
            let Ok(title) = elements[i].inner_text().await else {
                return Err(error::Errr::VecError);
            };
            let Some(title_text) = title else {
                return Err(error::Errr::VecError);
            };
            let Ok(atag) = elements[i].find_element("a").await else {
                return Err(error::Errr::Null);
            };
            let Ok(url) = atag.attribute("href").await else {
                return Err(error::Errr::VecError);
            };
            let Some(url_text) = url else {
                return Err(error::Errr::VecError);
            };
            datas.push(resultdata::ResultData::new(&title_text, &url_text));
        }
        print!("{:?}", elements);
        return Ok(datas);
    }
}
