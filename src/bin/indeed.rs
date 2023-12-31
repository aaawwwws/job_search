#![no_main]
use anyhow::{anyhow, Result};
use chromiumoxide::{Browser, BrowserConfig, Element, Page};
use futures::StreamExt;
#[path = "./region.rs"]
pub mod region;
#[path = "./resultdata.rs"]
pub mod resultdata;

pub struct Indeed {
    url: String,
}

impl Indeed {
    pub fn new(data: &region::Region) -> Self {
        let search_url = String::from("https://jp.indeed.com/jobs?q=&l=");
        let u = format!("{}{}", search_url, data.get_region());
        return Self { url: u };
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

    //ランメソッド
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

        let Ok(elements) = page.find_elements("div.job_seen_beacon").await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let Ok(atag_texts) = self.tag_search(&elements, "h2 > a").await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let com_tag = String::from("span.css-1x7z1ps.eu4oa1w0");

        let Ok(com_names) = self.tag_search(&elements, &com_tag).await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let location_tag = String::from("div.css-t4u72d.eu4oa1w0");

        let Ok(locations) = self.tag_search_lacation(&elements, &location_tag).await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let amount_tag = String::from("div.salary-snippet > span");

        let Ok(amounts) = self.tag_search(&elements, &amount_tag).await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let employment_status_tag =
            String::from("div.heading6.tapItem-gutter.metadataContainer > div[class='metadata'] ");

        let Ok(employment_status) = self.tag_search(&elements, &employment_status_tag).await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        print!("{:?}", employment_status);

        let Ok(descriptions) = self.tag_search(&elements, "div.job-snippet").await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let Ok(atag_urls) = self.tag_search_at(&elements, "h2 > a", "href").await else {
            return Err(anyhow!("タグが見つかりませんでした"));
        };

        let mut vec: Vec<resultdata::ResultData> = vec![];
        let indeed = String::from("https://jp.indeed.com");
        for i in 0..elements.len() {
            vec.push(resultdata::ResultData::new(
                &atag_texts[i],
                &com_names[i],
                &locations[i],
                &amounts[i],
                &employment_status[i],
                &descriptions[i],
                format!("{}{}", indeed, &atag_urls[i]).as_str(),
            ))
        }
        return Ok(vec);
    }
}
