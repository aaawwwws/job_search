#![no_main]
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ResultData {
    title: String,
    com_name: String,
    location: String,
    amount: String,
    employment_status: String,
    description: String,
    url: String,
}

impl ResultData {
    pub fn new(
        title: &str,
        com_name: &str,
        location: &str,
        amount: &str,
        employment_status: &str,
        description: &str,
        url: &str,
    ) -> Self {
        return Self {
            title: title.to_string(),
            com_name: com_name.to_string(),
            location: location.to_string(),
            amount: amount.to_string(),
            employment_status: employment_status.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        };
    }
    pub fn get_title(&self) -> &str {
        return &self.title;
    }
    pub fn get_com_name(&self) -> &str {
        return &self.com_name;
    }
    pub fn get_location(&self) -> &str {
        return &self.location;
    }
    pub fn get_amount(&self) -> &str {
        return &self.amount;
    }
    pub fn get_employment_status(&self) -> &str {
        return &self.employment_status;
    }
    pub fn get_description(&self) -> &str {
        return &self.description;
    }
    pub fn get_url(&self) -> &str {
        return &self.url;
    }
}
