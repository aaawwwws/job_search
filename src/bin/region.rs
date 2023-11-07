pub struct Region {
    region: String,
}
impl Region {
    pub fn new(region: &str) -> Self {
        Self {
            region: region.to_string(),
        }
    }
    pub fn get_region(&self) -> &str {
        return &self.region;
    }
}
