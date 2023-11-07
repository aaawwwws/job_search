pub struct ResultData {
    title: String,
    url: String,
}

impl ResultData {
    pub fn new(title: &str, url: &str) -> Self {
        return Self {
            title: title.to_string(),
            url: url.to_string(),
        };
    }
    pub fn get_title(&self) -> &str {
        return &self.title;
    }
    pub fn get_url(&self) -> &str {
        return &self.url;
    }
}
