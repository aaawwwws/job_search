#![no_main]
use anyhow::anyhow;
use chromiumoxide::{Browser, BrowserConfig, Element, Page};
use futures::StreamExt;
#[path = "./region.rs"]
pub mod region;
#[path = "./resultdata.rs"]
mod resultdata;
pub struct Stanb {
    url: String,
}

impl Stanb {
    pub fn new(data: &region::Region) -> Self {
        let search_url = String::from("https://jp.stanby.com/search?l=");
        Self {
            url: format!("{}{}", &search_url, data.get_region()),
        }
    }

    async fn tag_search_lacation(
        &self,
        elements: &Vec<Element>,
        tag: &str,
    ) -> anyhow::Result<Vec<String>> {
        let mut vec: Vec<String> = vec![];
        for i in 0..elements.len() {
            if let Ok(area) = elements[i].find_element(tag.to_string()).await {
                if let Ok(area_text) = area.inner_text().await {
                    vec.push(area_text.unwrap());
                }
            } else {
                let Ok(test) = elements[i].find_element("div.css-ozny1x.eu4oa1w0").await else {
                    return Err(anyhow!("タグが見つかりません"));
                };
                let Ok(area_text) = test.inner_text().await else {
                    return Err(anyhow!("タグが見つかりません"));
                };
                vec.push(area_text.unwrap());
            }
        }
        return Ok(vec);
    }

    //タグのインナーテキストから値を取り出したい場合
    async fn tag_search(&self, elements: &Vec<Element>, tag: &str) -> anyhow::Result<Vec<String>> {
        let mut vec: Vec<String> = vec![];
        const TEST: &str = "Not Found";
        for i in elements.iter() {
            if let Ok(atag) = i.find_element(tag.to_string()).await {
                if let Ok(title_text) = atag.inner_text().await {
                    let title_unwrap = title_text.unwrap();
                    vec.push(title_unwrap);
                };
            } else {
                vec.push(TEST.to_string());
            }
        }
        return Ok(vec);
    }

    async fn tag_search_s(&self, elements: &Vec<Element>, tag: &str) -> anyhow::Result<LocAmoEmp> {
        let mut locations: Vec<String> = vec![];
        let mut amounts: Vec<String> = vec![];
        let mut employment_status: Vec<String> = vec![];
        for i in 0..elements.len() {
            let Ok(tag) = elements[i].find_elements(tag).await else {
                return Err(anyhow!("タグが見つかりませんでした"));
            };
            for j in 0..tag.len() {
                let Ok(text) = tag[j].inner_text().await else {
                    return Err(anyhow!("タグが見つかりませんでした"));
                };
                match j % 3 {
                    0 => locations.push(text.unwrap()),
                    1 => amounts.push(text.unwrap()),
                    _ => employment_status.push(text.unwrap()),
                }
            }
        }
        return Ok(LocAmoEmp::new(locations, amounts, employment_status));
    }

    //タグの属性から値を取り出したい場合.
    async fn tag_search_at(
        &self,
        elements: &Vec<Element>,
        tag: &str,
        attribute: &str,
    ) -> anyhow::Result<Vec<String>> {
        let mut vec: Vec<String> = vec![];
        for i in elements.iter() {
            let Ok(taag) = i.find_element(tag.to_string()).await else {
                return Err(anyhow!("タグが見つかりません"));
            };
            let Ok(attr) = taag.attribute(attribute.to_string()).await else {
                return Err(anyhow!("タグが見つかりません"));
            };
            let tag_text = attr.unwrap();

            vec.push(tag_text);
        }
        return Ok(vec);
    }

    pub async fn run(&self) -> anyhow::Result<Vec<resultdata::ResultData>> {
        let Ok((browser, mut handler)) = Browser::launch(
            BrowserConfig::builder()
                .window_size(1920, 1080)
                .with_head()
                .build()
                .unwrap(),
        )
        .await
        else {
            return Err(anyhow!("ブラウザが開けませんでした"));
        };

        tokio::spawn(async move {
            loop {
                let _event = handler.next().await.unwrap();
            }
        });

        let Ok(page) = browser.new_page(&self.url).await else {
            return Err(anyhow!("ページが開けませんでした"));
        };

        let Ok(_) = page.wait_for_navigation().await else {
            return Err(anyhow!("ページが開けませんでした"));
        };

        let Ok(elements) = page.find_elements(".job-list-item").await else {
            return Err(anyhow!("タグが取得できませんでした。"));
        };

        let title = self.tag_search(&elements, "h2").await?;

        let com_names = self.tag_search(&elements, "p[class='company']").await?;

        let lae = self.tag_search_s(&elements, "p.property-item-main").await?;

        print!(
            "{:?}{:?}{:?}",
            lae.get_location(),
            lae.get_amout(),
            lae.get_employment_status()
        );
        lae.amounts;

        let mut v = vec![];
        return Ok(v);
    }
}

struct LocAmoEmp {
    locations: Vec<String>,
    amounts: Vec<String>,
    employment_status: Vec<String>,
}

impl LocAmoEmp {
    fn new(locations: Vec<String>, amounts: Vec<String>, employment_status: Vec<String>) -> Self {
        Self {
            locations,
            amounts,
            employment_status,
        }
    }
    fn get_location(&self) -> &Vec<String> {
        return &self.locations;
    }
    fn get_amout(&self) -> &Vec<String> {
        return &self.amounts;
    }
    fn get_employment_status(&self) -> &Vec<String> {
        return &self.employment_status;
    }
}
