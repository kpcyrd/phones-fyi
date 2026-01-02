use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Vendor {
    pub id: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub codename: String,
    pub name: String,
    pub vendor_id: String,
    pub release_date: String,
    // pub discontinued: bool,
}
