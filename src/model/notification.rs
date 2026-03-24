use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(create = "rocket::serde")]
pub struct Notification {
    pub product_title: String,
    pub product_type: String,
    pub product_url: String,
    pub subscriber_name: String,
    pub status: String
}